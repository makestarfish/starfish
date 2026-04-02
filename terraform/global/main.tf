provider "tfe" {}

resource "tfe_variable_set" "global" {
  name         = "Global Settings"
  description  = "For variables that are used in multiple or all environments"
  organization = "starfish-labs"
  global       = true
}

resource "tfe_variable" "ghcr_username" {
  key             = "ghcr_username"
  category        = "terraform"
  description     = "GitHub username for GHCR authentication"
  sensitive       = true
  variable_set_id = tfe_variable_set.global.id
}

resource "tfe_variable" "ghcr_auth_token" {
  key             = "ghcr_auth_token"
  description     = "Personal GitHub access token with read:packages scope"
  category        = "terraform"
  sensitive       = true
  variable_set_id = tfe_variable_set.global.id
}

resource "tfe_variable" "render_api_key" {
  key             = "RENDER_API_KEY"
  description     = "Render API key"
  category        = "env"
  sensitive       = true
  variable_set_id = tfe_variable_set.global.id
}

resource "tfe_variable" "render_owner_id" {
  key             = "RENDER_OWNER_ID"
  value           = "usr-cfq7e8qrrk08lt47viig"
  category        = "env"
  description     = "Render owner ID"
  variable_set_id = tfe_variable_set.global.id
}
