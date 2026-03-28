resource "render_registry_credential" "ghcr" {
  name = "Registry Credentials for GHCR"
  registry = "GITHUB"
  username = var.ghcr_username
  auth_token = var.ghcr_auth_token
}

resource "render_project" "starfish" {
  name = "Starfish"

  environments = {
    "Production" = {
      name = "Production"
      protected_status = "unprotected"
    }
  }

  depends_on = [render_registry_credential.ghcr]
}
