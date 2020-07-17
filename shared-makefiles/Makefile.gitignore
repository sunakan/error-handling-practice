################################################################################
# 変数
################################################################################
IGNORE_OS      := linux,macos,windows
IGNORE_EDITOR  := vim,emacs,intellij+all,visualstudiocode
IGNORE_LIST    := rust,$(IGNORE_OS),$(IGNORE_EDITOR)
GIT_IGNORE_URL := https://www.toptal.com/developers/gitignore/api/$(IGNORE_LIST)

################################################################################
# タスク
################################################################################
.gitignore:
	curl --output .gitignore $(GIT_IGNORE_URL)

.PHONY: setup-gitignore
setup-gitignore: .gitignore ## .gitignoreをsetup
