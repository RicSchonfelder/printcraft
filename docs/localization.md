# Interface language

Choose **Edit > Preferences > Interface language** and select **English**, **日本語** or **Português (Brasil)**. The change applies immediately and persists between launches. Command ids, document contents and file names are unchanged.

The control channel exposes the setting through `ui.set`:

```json
{"method":"ui.set","params":{"key":"language","value":"ja"}}
```

Other codes follow the same shape: `{"key":"language","value":"pt-br"}` switches the interface to Brazilian Portuguese.

`ui.state` reports `language` as `en`, `ja` or `pt-br` (BCP-47 lower case). Unknown language codes return an error without changing the current setting. Old preferences default to English.

This first translation pass covers the main menu and core registered menu commands. The Brazilian Portuguese (`pt-br`, endonym **Português (Brasil)**) table carries exactly the same keys as the Japanese one, with the same placeholders and punctuation as the English source. Untranslated labels use English. Vertical Japanese PDF rendering is an existing viewer feature; this change does not add vertical text editing.

Japanese interface text uses BIZ UDPGothic from [craft-fonts](https://github.com/storytold/craft-fonts), an optional build input that every release includes (`CRAFT_FONTS_DIR`; see the README). A build made without it has no Japanese face, so Japanese labels show replacement boxes. Brazilian Portuguese uses the standard Latin faces, so it needs no extra fonts.
