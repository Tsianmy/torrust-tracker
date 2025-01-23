use std::sync::Arc;

use tokio::sync::RwLock;

use crate::core::authentication::handler::KeysHandler;
use crate::core::authentication::service::AuthenticationService;
use crate::core::databases::Database;
use crate::core::statistics::event::sender::Sender;
use crate::core::statistics::repository::Repository;
use crate::core::torrent::manager::TorrentsManager;
use crate::core::torrent::repository::in_memory::InMemoryTorrentRepository;
use crate::core::torrent::repository::persisted::DatabasePersistentTorrentRepository;
use crate::core::whitelist::manager::WhiteListManager;
use crate::core::{whitelist, Tracker};
use crate::servers::udp::server::banning::BanService;

pub struct AppContainer {
    pub database: Arc<Box<dyn Database>>,
    pub tracker: Arc<Tracker>,
    pub keys_handler: Arc<KeysHandler>,
    pub authentication_service: Arc<AuthenticationService>,
    pub whitelist_authorization: Arc<whitelist::authorization::Authorization>,
    pub ban_service: Arc<RwLock<BanService>>,
    pub stats_event_sender: Arc<Option<Box<dyn Sender>>>,
    pub stats_repository: Arc<Repository>,
    pub whitelist_manager: Arc<WhiteListManager>,
    pub in_memory_torrent_repository: Arc<InMemoryTorrentRepository>,
    pub db_torrent_repository: Arc<DatabasePersistentTorrentRepository>,
    pub torrents_manager: Arc<TorrentsManager>,
}
