#!/usr/bin/bash

test_type=$1
test_canister_name=$2

if [[ $test_type == "unit" ]]; then

    if [[ $test_canister_name == "accounts" ]]; then

        echo "Running Accounts Uint Tests"

        jest -c ./jest.config.ts --testPathPattern='tests/unit/accounts'

    elif [[ $test_canister_name == "insurance" ]]; then

        echo "Running Insurance Uint Tests"

        jest --testPathPattern='tests/unit/insurance'

    fi

elif [[ $test_type == "integration" ]]; then

    if [[ $test_canister_name == "accounts" ]]; then

        echo "Running Accounts Integration Tests"

    elif [[ $test_canister_name == "insurance" ]]; then

        echo "Running Insurance Integration Tests"

        jest --testPathPattern='tests/integration/insurance/1_insurance.spec.ts'

        jest --testPathPattern='tests/integration/insurance/2_insurance.spec.ts'

    fi

elif [[ $test_type == "all" ]]; then

    echo "Running Accounts Uint Tests"

    jest -c ./jest.config.ts --testPathPattern='tests/unit/accounts'

    echo "Running Insurance Uint Tests"

    jest --testPathPattern='tests/unit/insurance'

    echo "Running Insurance Integration Tests"

    jest --testPathPattern='tests/integration/insurance/1_insurance.spec.ts'

    jest --testPathPattern='tests/integration/insurance/2_insurance.spec.ts'

fi
