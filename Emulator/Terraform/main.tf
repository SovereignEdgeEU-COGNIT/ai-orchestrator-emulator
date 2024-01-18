terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "3.0.2"
    }
  }
}

provider "docker" {
  host = "npipe:////./pipe/docker_engine"
}

resource "docker_image" "cognit" {
  name = "example-img:latest"
  build {
    context = "../emulated_host"
  }
}

resource "docker_container" "cognit" {
  image = docker_image.cognit.image_id
  name  = "cognit-host"
  tty = true
  publish_all_ports = false
  # ports {
  #   internal = 80
  #   external = 8080
  # }
}


resource "docker_container" "cognit2" {
  image = docker_image.cognit.image_id
  name  = "cognit-host2"
  tty = true
  publish_all_ports = false
  # ports {
  #   internal = 80
  #   external = 8080
  # }
}
