
target "dapp" {
}

variable "TAG" {
  default = "devel"
}

variable "DOCKER_ORGANIZATION" {
  default = "webchefs"
}

variable "DAPP_NAME" {
  default = "parking-dapp"
}

target "server" {
  tags = ["${DOCKER_ORGANIZATION}/${DAPP_NAME}:${TAG}-server"]
}

target "console" {
  tags = ["${DOCKER_ORGANIZATION}/${DAPP_NAME}:${TAG}-console"]
}

target "machine" {
  tags = ["${DOCKER_ORGANIZATION}/${DAPP_NAME}:${TAG}-machine"]
}
