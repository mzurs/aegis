#!/usr/bin/bash

test_type=$1

if [[ $test_type == "unit" ]]; then

    echo "Running Accounts Uint Tests"

    jest -c ./jest.config.ts --testPathPattern='tests/unit/accounts'

    echo "Running Insurance Uint Tests"

    jest --testPathPattern='tests/unit/insurance'

elif [[ $test_type == "integration" ]]; then

    echo "Running Insurance Integration Tests"

    jest --testPathPattern='tests/integration/insurance/1_insurance.spec.ts'

    jest --testPathPattern='tests/integration/insurance/2_insurance.spec.ts'

fi
