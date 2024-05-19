---@module 'clico.data'
local data = setmetatable({}, {
  __index = function(_, key)
    return require('clico.data')[key]
  end,
})

local M = {}

---@param filetype string?
---@return integer?
local function idx_by_filetype(filetype)
  return data.idx_by_filetype[filetype]
end

---@param filename string?
---@return integer?
local function idx_by_filename(filename)
  return data.idx_by_filename[filename]
end

---@param filename string?
---@return integer?
local function idx_by_extension(filename)
  if not filename then
    return
  end
  local ext = filename:match('%.(%g+)$')
  if not ext then
    return
  end
  local idx = data.idx_by_extension[ext]
  if not idx then
    return idx_by_extension(ext)
  end
  return idx
end

---@param opts { path?: string, ft?: string }
---@return { icon: string, hl: string, default: boolean }
function M.get(opts)
  local filename = opts.path and opts.path:lower():match('([^/\\]+)$') or nil
  local idx = idx_by_filetype(opts.ft)
    or idx_by_filename(filename)
    or idx_by_extension(filename)
  local t = idx and data.icon_highlight_list[idx] or data.icon_highlight_list[1]
  return { icon = t[1], hl = t[2], default = not idx }
end

function M.nvim_web_devicons()
  local devicon = {}
  ---@diagnostic disable: unused-local

  function devicon.setup() end

  function devicon.has_loaded()
    return true
  end

  function devicon.get_icon(filename, extension, opts)
    local icon = M.get({ path = filename })
    if opts and not opts.default and icon.default then
      return
    end
    return icon.icon, icon.hl
  end

  function devicon.get_icon_by_filetype(filetype, opts)
    local icon = M.get({ ft = filetype })
    if opts and not opts.default and icon.default then
      return
    end
    return icon.icon, icon.hl
  end

  ---@diagnostic enable
  return devicon
end

return M
