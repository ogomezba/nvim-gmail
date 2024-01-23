local M = {}

local gmail_buffer
local email_headers

local load_messages = function(channel)
	email_headers = vim.rpcrequest(channel, "get_last_emails")
	local lines = {}

	if not email_headers then
		vim.notify("Empty inbox. No emails to display")
		return
	end

	for idx, header in ipairs(email_headers) do
		lines[idx] = string.format("From: %s || %s || %s", header.from, header.subject, header.date)
	end

	vim.api.nvim_buf_set_option(gmail_buffer, "modifiable", true)
	vim.api.nvim_buf_set_lines(gmail_buffer, 0, -1, false, lines)
	vim.api.nvim_buf_set_option(gmail_buffer, "modifiable", false)

	vim.keymap.set("n", "<CR>", function()
		M.get_email_body(channel, email_headers[vim.fn.line(".")].uid)
	end, { buffer = gmail_buffer })
	vim.keymap.set("n", "q", "<cmd>bd<cr>", { buffer = gmail_buffer })

	vim.api.nvim_set_current_buf(gmail_buffer)
end

M.open_last_emails = function(channel)
	if gmail_buffer and vim.api.nvim_buf_is_loaded(gmail_buffer) then
		vim.api.nvim_set_current_buf(gmail_buffer)
		return
	end

	gmail_buffer = vim.api.nvim_create_buf(false, true)
	load_messages(channel)
end

M.open_send_email = function(channel)
	local buffer = vim.api.nvim_create_buf(false, true)

	local send_email = function()
		local subject = vim.fn.input("subject: ")
		local to = vim.fn.input("to: ")
		local body = vim.fn.join(vim.api.nvim_buf_get_lines(buffer, 0, -1, false), "\n")

		vim.rpcrequest(channel, "send_email", to, subject, body)

		vim.api.nvim_buf_delete(buffer, { force = true })
	end

	vim.keymap.set("n", "cc", send_email, { buffer = buffer })
	vim.api.nvim_set_current_buf(buffer)
end

M.get_email_body = function(channel, uid)
	local body = vim.rpcrequest(channel, "get_email_body", uid)

	local buffer = vim.api.nvim_create_buf(false, true)
	vim.api.nvim_buf_set_lines(buffer, 0, -1, false, vim.split(body, "\n"))
	vim.api.nvim_buf_set_option(buffer, "modifiable", false)

	vim.keymap.set("n", "q", "<cmd>bd<cr>", { buffer = buffer })

	vim.api.nvim_set_current_buf(buffer)
end

return M
