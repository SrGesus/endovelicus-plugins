# Endovelicus Plugins
A collection of plugins for use in the [Endovelicus](https://github.com/SrGesus/endovelicus) app.

## To Do:
- [ ] Github workflows for:
  - [ ] Building
  - [ ] Uploading Releases
- [ ] Currency rate fetching plugin
  - [ ] Make use of a api to get currency rates and names like [frankfurter](https://www.frankfurter.app/) (unfortunately no symbols) or [freecurrencyapi](https://freecurrencyapi.com/docs/latest#request-parameters) (symbols but needs api key)
  - [ ] Endpoint for refreshing currencies

## Plugins

Plugin implementations should include:
- validate() -> bool
Returning whether the plugin configuration is valid

### Currency Api
Plugin that fetches data about currencies from an api and adds it to Endovelicus.
