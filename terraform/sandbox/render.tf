resource "render_registry_credential" "ghcr" {
  name       = "Registry Credentials for GHCR"
  registry   = "GITHUB"
  username   = var.ghcr_username
  auth_token = var.ghcr_auth_token
}

resource "render_project" "starfish" {
  name = "Starfish"

  environments = {
    "Sandbox" = {
      name             = "Sandbox"
      protected_status = "unprotected"
    }
  }

  depends_on = [render_registry_credential.ghcr]
}

resource "render_postgres" "database" {
  environment_id = render_project.starfish.environments["Sandbox"].id
  name           = "database"
  database_name  = "starfish_database"
  database_user  = "starfish_database_user"
  plan           = "free"
  region         = "ohio"
  version        = "18"
  depends_on     = [render_project.starfish, render_registry_credential.ghcr]
}

resource "render_env_group" "stripe" {
  environment_id = render_project.starfish.environments["Sandbox"].id
  name           = "stripe"
  env_vars = {
    STRIPE_SECRET_KEY     = { value = var.stripe_secret_key_sandbox }
    STRIPE_WEBHOOK_SECRET = { value = var.stripe_webhook_secret_sandbox }
  }
}

resource "render_env_group" "starfish" {
  environment_id = render_project.starfish.environments["Sandbox"].id
  name           = "starfish"
  env_vars = {
    JWT_SECRET     = { value = var.starfish_jwt_secret_sandbox }
    RESEND_API_KEY = { value = var.starfish_resend_api_key_sandbox }
  }
}

# =============================================================================
# Service data source
#
# We read the current image digest from Render to avoid stale state in
# Terraform. The service ID is hardcoded because it comes from an output.
#
# First-time setup: create the service first without the data source — use
# "ghcr.io/makestarfish/starfish" as the image URL — then add the data source
# with the service ID from `terraform output starfish_service_id`.
# =============================================================================

locals {
  starfish_service_id = "srv-d76kc49aae7s73c78q20"
}

data "render_web_service" "starfish" {
  id = local.starfish_service_id
}

resource "render_web_service" "starfish" {
  environment_id     = render_project.starfish.environments["Sandbox"].id
  name               = "starfish"
  plan               = "starter"
  region             = "ohio"
  pre_deploy_command = "starfish_migrate"

  runtime_source = {
    image = {
      image_url              = split("@", data.render_web_service.starfish.runtime_source.image.image_url)[0]
      digest                 = data.render_web_service.starfish.runtime_source.image.digest
      registry_credential_id = render_registry_credential.ghcr.id
    }
  }

  env_vars = {
    DATABASE_URL     = { value = render_postgres.database.connection_info.internal_connection_string }
    WEBSITE_BASE_URL = { value = "https://starfish.so" }
  }
}

resource "render_env_group_link" "stripe" {
  env_group_id = render_env_group.stripe.id
  service_ids  = [render_web_service.starfish.id]
}

resource "render_env_group_link" "starfish" {
  env_group_id = render_env_group.starfish.id
  service_ids  = [render_web_service.starfish.id]
}
