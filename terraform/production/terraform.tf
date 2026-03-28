terraform {
  required_version = ">= 1.14"

  cloud {
    organization = "starfish-labs"

    workspaces {
      name = "starfish"
    }
  }

  required_providers {
    render = {
      source  = "render-oss/render"
      version = "1.8.0"
    }
  }
}
