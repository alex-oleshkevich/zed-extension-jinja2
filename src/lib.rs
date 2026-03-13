use zed_extension_api::{self as zed, LanguageServerId};

struct JinjaExtension {}

impl zed::Extension for JinjaExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: String::from("/home/alex/projects/jinja2-lsp/run-lsp.sh"),
            args: vec![],
            env: vec![],
        })
    }
}

zed::register_extension!(JinjaExtension);
