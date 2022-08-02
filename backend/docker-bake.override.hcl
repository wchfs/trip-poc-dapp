
target "dapp" {
}

target "server" {
  tags = ["cartesi/dapp:trip_poc-devel-server"]
}

target "console" {
  tags = ["cartesi/dapp:trip_poc-devel-console"]
}

target "machine" {
  tags = ["cartesi/dapp:trip_poc-devel-machine"]
}
