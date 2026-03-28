# Starfish Infrastructure as Code (IaC)

Starfish uses Terraform to manage and provision its cloud infrastructure. The Terraform state, secrets and runs are managed on [HCP Terraform](https://developer.hashicorp.com/terraform/cloud-docs).

## Infrastructure details

Starfish uses...

- [Render](https://render.com) for webservices and databases
- [Google Cloud Storage](https://cloud.google.com/storage) for file storage
- [GHCR](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry) to store container images