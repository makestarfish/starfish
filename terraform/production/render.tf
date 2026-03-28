resource "render_registry_credential" "ghcr" {
  name       = "Registry Credentials for GHCR"
  registry   = "GITHUB"
  username   = var.ghcr_username
  auth_token = var.ghcr_auth_token
}

resource "render_project" "starfish" {
  name = "Starfish"

  environments = {
    "Production" = {
      name             = "Production"
      protected_status = "unprotected"
    }
  }

  depends_on = [render_registry_credential.ghcr]
}

resource "render_postgres" "database" {
  environment_id = render_project.starfish.environments["Production"].id
  name           = "database"
  database_name  = "starfish_database"
  database_user  = "starfish_database_user"
  plan           = "free"
  region         = "ohio"
  version        = "18"
  depends_on     = [render_project.starfish, render_registry_credential.ghcr]
}