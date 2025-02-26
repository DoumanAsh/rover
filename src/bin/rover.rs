use robot_panic::setup_panic;
use rover::cli::Rover;

#[calm_io::pipefail]
fn main() -> std::io::Result<()> {
    setup_panic!(Metadata {
        name: rover::PKG_NAME.into(),
        version: rover::PKG_VERSION.into(),
        authors: rover::PKG_AUTHORS.into(),
        homepage: rover::PKG_HOMEPAGE.into(),
        repository: rover::PKG_REPOSITORY.into()
    });
    Rover::run_from_args()
}
