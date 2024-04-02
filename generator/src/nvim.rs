use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use heck::ToUpperCamelCase;

pub fn generate(mut output: super::Output) -> Result<()> {
    for name in &mut output.names {
        *name = format!("Clico{}", name.to_upper_camel_case());
    }
    let dark_colors = output.names.iter().zip(output.dark_colors.iter()).fold(
        String::from("\n"),
        |acc, (name, color)| {
            format!(
                "{acc}  {name} = {{ '{}', {} }},\n",
                color.hex(),
                color.ansi()
            )
        },
    );
    let light_colors = output.names.iter().zip(output.light_colors.iter()).fold(
        String::from("\n"),
        |acc, (name, color)| {
            format!(
                "{acc}  {name} = {{ '{}', {} }},\n",
                color.hex(),
                color.ansi()
            )
        },
    );
    let icon_highlight_list = output
        .icons
        .iter()
        .zip(output.names.iter())
        .fold(String::from("\n"), |acc, (icon, name)| {
            format!("{acc}  {{ '{icon}', '{name}' }},\n")
        });
    let idx_by_filename = output
        .idx_by_filename
        .iter()
        .fold(String::from("\n"), |acc, (filename, idx)| {
            format!("{acc}  ['{filename}'] = {},\n", idx + 1)
        });
    let idx_by_filetype = output
        .idx_by_filetype
        .iter()
        .fold(String::from("\n"), |acc, (filetype, idx)| {
            format!("{acc}  ['{filetype}'] = {},\n", idx + 1)
        });
    let idx_by_extension = output
        .idx_by_extension
        .iter()
        .fold(String::from("\n"), |acc, (extension, idx)| {
            format!("{acc}  ['{extension}'] = {},\n", idx + 1)
        });
    let contents = format!(
        "-- generated from input.toml

local colors = {{}}

colors.dark = {{{dark_colors}}}

colors.light = {{{light_colors}}}

local function setup_highlights()
  local hl = vim.api.nvim_set_hl
  for hl_name, color in pairs(colors[vim.o.background]) do
    hl(0, hl_name, {{ fg = color[1], ctermfg = color[2], force = true }})
  end
end

vim.api.nvim_create_autocmd('ColorScheme', {{ callback = setup_highlights }})
if vim.v.vim_did_enter == 1 then
  setup_highlights()
end

local M = {{}}

M.icon_highlight_list = {{{icon_highlight_list}}}

M.idx_by_filename = {{{idx_by_filename}}}

M.idx_by_filetype = {{{idx_by_filetype}}}

M.idx_by_extension = {{{idx_by_extension}}}

return M
"
    );
    File::create("./lua/clico/data.lua")
        .context("failed to open `lua/clico/data.lua`")?
        .write_all(contents.as_bytes())?;
    Ok(())
}
