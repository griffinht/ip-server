#!/bin/bash

cargo metadata --no-deps --format-version 1 | jq --raw-output .packages[]."$1"