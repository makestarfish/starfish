output "render_project_id" {
  description = "The ID of the Render project"
  value       = render_project.starfish.id
}

output "render_postgres_id" {
  description = "The ID of the Render Postgres"
  value       = render_postgres.database.id
}

output "starfish_service_id" {
  description = "The ID of the Render web service"
  value       = render_web_service.starfish.id
}

output "starfish_service_url" {
  description = "The URL of the Render web service"
  value       = render_web_service.starfish.url
}
