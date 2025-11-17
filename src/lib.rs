use std::fs;
use zed_extension_api as zed;

// rustup default stable
// rustup target add wasm32-wasip2
// cargo build --target=wasm32-wasip2

struct CMakeTools {
    lsp_path: String,
}

impl CMakeTools {
    fn download_lsp(language_server_id: &zed::LanguageServerId) -> zed::Result<String> {
        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "Decodetalkers/neocmakelsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "neocmakelsp-{arch}-{os}",
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu", // Choose GNU by default
                zed::Os::Windows => "pc-windows-msvc.exe",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => {
                    if platform == zed::Os::Windows {
                        return Err("unsupported platform aarch64".into());
                    }
                    "aarch64"
                }
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 => return Err("unsupported platform x86".into()),
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;
        let binary_path = format!("neocmakelsp-{}", release.version);

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;
        }

        // Clean up old LSP versions
        let entries =
            fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        for entry in entries {
            let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
            if entry.file_name().to_str() != Some(&binary_path) {
                fs::remove_dir_all(&entry.path()).ok();
            }
        }

        return Ok(binary_path);
    }
    fn get_lsp_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        // try to use cached path from previous initialization
        if fs::metadata(&self.lsp_path).map_or(false, |stat| stat.is_file()) {
            return Ok(self.lsp_path.clone());
        }

        // try to find neocmakelsp on $PATH
        if let Some(path) = worktree.which("neocmakelsp") {
            return Ok(path.clone());
        }

        // download latest release from GitHub
        let binary_path = CMakeTools::download_lsp(language_server_id)?;
        zed::make_file_executable(&binary_path)?;
        return Ok(binary_path.clone());
    }

    fn init(&mut self, worktree: &zed::Worktree) {}
}

impl zed::Extension for CMakeTools {
    fn new() -> Self {
        return Self {
            lsp_path: String::from(""),
        };
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        self.init(worktree);
        return Ok(zed::Command {
            command: self.get_lsp_path(language_server_id, worktree)?,
            args: vec![String::from("stdio")],
            env: Default::default(),
        });
    }
}

zed::register_extension!(CMakeTools);
