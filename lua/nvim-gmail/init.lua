local commands = require("nvim-gmail.commands")
local M = {}
local channel

local printe = function(message)
	vim.notify(message, 4)
end

M.setup = function(config)
	local function file_exists(path)
		local f = io.open(path, "r")
		if f ~= nil then
			io.close(f)
			return true
		else
			return false
		end
	end

	local is_valid_bin = config.bin_path and file_exists(config.bin_path)

	if not is_valid_bin then
		printe(string.format("Invalid path to Rust binary: %s", config.bin_path))
		return
	end

	channel = vim.fn.jobstart(config.bin_path, { rpc = true })

	vim.rpcrequest(channel, "start", config.username, config.pass)

	vim.api.nvim_create_user_command("GmailSendEmail", function()
		commands.open_send_email(channel)
	end, {})
	vim.api.nvim_create_user_command("GmailOpen", function()
		commands.open_last_emails(channel)
	end, {})
end

return M
