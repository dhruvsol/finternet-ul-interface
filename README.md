# Finternet UL Interface

Interface definitions and traits for implementing Finternet UL (Unified Ledger).

## ⚠️ Development Status

> **Warning**: This repository is under active development and is not production-ready. Use at your own risk.

## Overview

Finternet UL Interface provides a standardized set of interfaces and traits for building unified ledger systems. These definitions enable consistent implementation across different components of the Finternet ecosystem.

## Repository Structure

```
├── src/
│   ├── key/                  # Core interface for key
│   │   ├── ul_key.rs        # Key specs for the native key for the UL
│   │   ├── unique_key.rs    # Specs for the global finternet Keys
│   │   └── resolver.rs      # Specs for a module for resolving ul_key to unique_key and vice versa
│   │
│   └── storage/             # Core interface for storage

```
