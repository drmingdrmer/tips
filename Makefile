check-local-link:
	find ./tips/ -name \*.md -print0 | xargs -0 -n1 markdown-link-check -c link-check-config.json
