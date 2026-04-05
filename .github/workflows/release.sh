#!/bin/bash

SERVICE_ID="${1}"
IMAGE_URL="ghcr.io/makestarfish/starfish@${2}"

TIMEOUT=300 # 5 minutes
POOL_INTERVAL=5 # 5 seconds

check_deploy_status() {
  local service_id=$1
  local deploy_id=$2
  local response

  response=$(curl -s -X GET \
    --header "accept: application/json" \
    --header "authorization: Bearer ${RENDER_API_KEY}" \
    --url https://api.render.com/v1/services/${service_id}/deploys/${deploy_id} \
    | jq -r '.status' 2>/dev/null
  )

  echo "$response"
}

echo "🚀 STARTING DEPLOYMENT"

response=$(curl -s -X POST \
  --header "accept: application/json" \
  --header "content-type: application/json" \
  --header "authorization: Bearer ${RENDER_API_KEY}" \
  --data "{ \"imageUrl\": \"${IMAGE_URL}\" }" \
  "https://api.render.com/v1/services/${SERVICE_ID}/deploys"
)

deploy_id=$(echo "$response" | jq -r '.id' 2>/dev/null)

if [[ -z "$deploy_id" || "$deploy_id" == "null" ]]
then
  echo "❌ Got error response from Render"
  echo "Response: ${response}"
  exit 1
fi

echo "⏳ Waiting for service update..."

start_time=$(date +%s)
complete=false

while [[ $complete == false ]]
do
  status=$(check_deploy_status "$SERVICE_ID" "$deploy_id")
  complete=true

  case "$status" in
    "live")
      echo "✅ service is live!"
      ;;
    "build_failed"|"update_failed"|"pre_deploy_failed"|"canceled")
      echo "❌ failed with status ${status}"
      exit 1
      ;;
    "created"|"queued"|"build_in_progress"|"update_in_progress"|"pre_deploy_in_progress")
      echo "🔄 in progress (status is '${status}')"
      complete=false
      ;;
    "deactivated")
      echo "⚠️ service is deactivated"
      complete=false
      ;;
    *)
      echo "⚠️ unknown status ${status}"
      complete=false
      ;;
  esac

  if [[ $complete == false ]]
  then
    current_time=$(date +%s)
    elapsed=$((current_time - start_time))

    if [[ $elapsed -gt $TIMEOUT ]]
    then
      echo "Timeout after ${TIMEOUT} seconds"
      exit 1
    fi

    echo "Checking again in ${POOL_INTERVAL} seconds..."
    sleep $POOL_INTERVAL
  fi
done

echo "🎉 DEPLOYMENT COMPLETED SUCCESSFULLY!"
