// Vérification : au moins une interface doit être activée
#[cfg(not(any(
    feature = "gui",
    feature = "cli",
    feature = "tui",
    feature = "web",
    feature = "api",
    feature = "ask"
)))]
compile_error!(
    "You must enable at least one interface: 'gui', 'cli', 'tui', 'web', 'api', or 'ask'."
);

// Vérification : une seule interface à la fois
#[cfg(all(
    feature = "gui",
    any(
        feature = "cli",
        feature = "tui",
        feature = "web",
        feature = "api",
        feature = "ask"
    )
))]
compile_error!("You cannot enable 'gui' with other interfaces.");

#[cfg(all(
    feature = "cli",
    any(feature = "gui", feature = "tui", feature = "web", feature = "api",)
))]
compile_error!("You cannot enable 'cli' with other interfaces.");

#[cfg(all(
    feature = "tui",
    any(
        feature = "gui",
        feature = "cli",
        feature = "web",
        feature = "api",
        feature = "ask"
    )
))]
compile_error!("You cannot enable 'tui' with other interfaces.");

#[cfg(all(
    feature = "web",
    any(
        feature = "gui",
        feature = "cli",
        feature = "tui",
        feature = "api",
        feature = "ask"
    )
))]
compile_error!("You cannot enable 'web' with other interfaces.");

#[cfg(all(
    feature = "api",
    any(
        feature = "gui",
        feature = "cli",
        feature = "tui",
        feature = "web",
        feature = "ask"
    )
))]
compile_error!("You cannot enable 'api' with other interfaces.");

#[cfg(all(
    feature = "ask",
    any(feature = "gui", feature = "tui", feature = "web", feature = "api")
))]
compile_error!("You cannot enable 'ask' with other interfaces.");

// Vérifications pour les VCS
#[cfg(all(
    feature = "git",
    any(feature = "mercurial", feature = "fossil", feature = "pijul")
))]
compile_error!("You cannot enable 'git' with other VCS features.");

#[cfg(all(
    feature = "mercurial",
    any(feature = "git", feature = "fossil", feature = "pijul")
))]
compile_error!("You cannot enable 'mercurial' with other VCS features.");

#[cfg(all(
    feature = "fossil",
    any(feature = "git", feature = "mercurial", feature = "pijul")
))]
compile_error!("You cannot enable 'fossil' with other VCS features.");

#[cfg(all(
    feature = "pijul",
    any(feature = "git", feature = "mercurial", feature = "fossil")
))]
compile_error!("You cannot enable 'pijul' with other VCS features.");

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "ask")]
pub mod ask;
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "fossil")]
pub mod fossil;
#[cfg(feature = "git")]
pub mod git;
#[cfg(feature = "gui")]
pub mod gui;
#[cfg(feature = "mercurial")]
pub mod mercurial;
#[cfg(feature = "pijul")]
pub mod pijul;
#[cfg(feature = "templates")]
pub mod tera;
#[cfg(feature = "tui")]
pub mod tui;

#[tokio::main]
async fn main() {
    #[cfg(feature = "gui")]
    {
        gui::run_gui().await;
    }

    #[cfg(feature = "cli")]
    {
        cli::run_cli().await;
    }

    #[cfg(feature = "tui")]
    {
        tui::run_tui().await;
    }

    #[cfg(feature = "web")]
    {
        tera::run_web().await;
    }
    #[cfg(feature = "api")]
    {
        api::run_api().await;
    }
    #[cfg(feature = "ask")]
    {
        ask::run_ask().await;
    }
}
