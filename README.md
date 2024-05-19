# clico

Libraries for CLI applications to detect the file icon and its color

## Dependencies

- Nerd Fonts

## Neovim plugin

lazy.nvim:

```lua
{
    'futsuuu/clico',
}
```

### Override nvim-web-devicons

```lua
-- Windows: $LOCALAPPDATA/nvim/lua/nvim-web-devicons.lua
-- Unix: $XDG_CONFIG_HOME/nvim/lua/nvim-web-devicons.lua

return require('clico').nvim_web_devicons()
```

## License

This repository is licensed under the [MIT License](./LICENSE).
