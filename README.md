# MixitForge
[![Build](https://github.com/RemcoDewlde/MixitForge/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/RemcoDewlde/MixitForge/actions/workflows/main.yml)

## Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

### Notes:
- MacOS build and updater probably not working because of the codesign, need an Apple developer account to sign the app, which is 99 bucks a year...

### TODO:
 - [ ] Auth / get token from Entra ID
 - [ ] Get user data from Entra ID
 - [ ] Config screen [Paths to repo's, App settings, etc]
 - [ ] Getting circuit breaker status
 - [ ] Getting azure function configs and writing them to `local.settings.json` with local dev settings applied (aka: `AzureWebJobsStorage` and `AzureWebJobsDashboard` set to `UseDevelopmentStorage=true`)
 - [ ] Dashboard of Release pipeline status
 - [ ] Dashboard of all the feature flags in use