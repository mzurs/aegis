#!/usr/bin/bash

set -e  # Stop execution on the first error


test_type=$1
test_canister_name=$2

if [[ $test_type == "unit" ]]; then
    
    if [[ $test_canister_name == "accounts" ]]; then
        
        echo "Running Accounts Unit Tests"
        
        jest -c ./jest.config.ts --testPathPattern='tests/unit/accounts'
        
        elif [[ $test_canister_name == "insurance" ]]; then
        
        echo "Running Insurance Unit Tests"
        
        jest --testPathPattern='tests/unit/insurance'
        
        elif [[ $test_canister_name == "main" ]]; then
        
        echo "Running Main Unit Tests"
        
        jest -c ./jest.config.ts --testPathPattern='tests/unit/main'
        
        
        elif [[ $test_canister_name == "options" ]]; then
        
        echo "Running Options Unit Tests"
        
        jest -c ./jest.config.ts --testPathPattern='tests/unit/options'
    fi
    
    elif [[ $test_type == "integration" ]]; then
    
    if [[ $test_canister_name == "accounts" ]]; then
        
        echo "Running Accounts Integration Tests"
        
        elif [[ $test_canister_name == "insurance" ]]; then
        
        echo "Running Insurance Integration Tests"
        
        jest --testPathPattern='tests/integration/insurance/1_insurance.spec.ts'
        
        jest --testPathPattern='tests/integration/insurance/2_insurance.spec.ts'
        
        
        elif [[ $test_canister_name == "options" ]]; then
        
        echo "Running Options Integration Tests"
        
        jest --testPathPattern='tests/integration/options/1_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/2_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/3_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/4_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/5_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/6_options.spec.ts'
        
        jest --testPathPattern='tests/integration/options/7_options.spec.ts'

        jest --testPathPattern='tests/integration/options/8_options.spec.ts' --detectOpenHandles

        elif [[ $test_canister_name == "main" ]]; then
        
        jest --testPathPattern='tests/integration/main/main.spec.ts'
        
    fi
    
    elif [[ $test_type == "all" ]]; then
    
    echo "Running Accounts Unit Tests"
    jest -c ./jest.config.ts --testPathPattern='tests/unit/accounts'
    
    echo "Running Insurance Unit Tests"
    jest --testPathPattern='tests/unit/insurance'
    
    echo "Running Main Unit Tests"
    jest -c ./jest.config.ts --testPathPattern='tests/unit/main'
    
    echo "Running Insurance Integration Tests"
    jest --testPathPattern='tests/integration/insurance/1_insurance.spec.ts'
    jest --testPathPattern='tests/integration/insurance/2_insurance.spec.ts'
    
fi
