#!/bin/bash

TOKEN=$1

cargo login "$TOKEN"
cargo publish