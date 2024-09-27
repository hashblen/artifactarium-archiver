//! Output format based on the format used by [Fribbels HSR Optimizer],
//! devised by [kel-z's HSR-Scanner].
//!
//! [Fribbels HSR Optimizer]: https://github.com/fribbels/hsr-optimizer
//! [kel-z's HSR-Scanner]: https://github.com/kel-z/HSR-Scanner
use std::collections::HashMap;

use crate::export::database::Database;
use protobuf::Enum;
use artifactarium::network::gen::command_id;
// use artifactarium::network::gen::proto::Avatar::Avatar as ProtoCharacter;
// use artifactarium::network::gen::proto::AvatarSkillTree::AvatarSkillTree as ProtoSkillTree;
// use artifactarium::network::gen::proto::Equipment::Equipment as ProtoLightCone;
// use artifactarium::network::gen::proto::GetAvatarDataScRsp::GetAvatarDataScRsp;
// use artifactarium::network::gen::proto::GetBagScRsp::GetBagScRsp;
// use artifactarium::network::gen::proto::GetMultiPathAvatarInfoScRsp::GetMultiPathAvatarInfoScRsp;
// use artifactarium::network::gen::proto::MultiPathAvatarInfo::MultiPathAvatarInfo;
// use artifactarium::network::gen::proto::MultiPathAvatarType::MultiPathAvatarType;
// use artifactarium::network::gen::proto::PlayerGetTokenScRsp::PlayerGetTokenScRsp;
// use artifactarium::network::gen::proto::Relic::Relic as ProtoRelic;
// use artifactarium::network::gen::proto::RelicAffix::RelicAffix;
use artifactarium::network::GameCommand;
use serde::{Deserialize, Serialize};
use tracing::{debug, info, info_span, instrument, trace, warn};

use crate::export::Exporter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Export {
    pub source: &'static str,
    pub build: &'static str,
    pub version: u32,
    pub uid: Option<u32>,
    pub achievements: Vec<u32>,
}

pub struct OptimizerExporter {
    uid: Option<u32>,
    achievements: Vec<u32>,
}

impl OptimizerExporter {
    pub fn new(database: Database) -> OptimizerExporter {
        OptimizerExporter {
            uid: None,
            achievements: vec![]
        }
    }

    pub fn set_uid(&mut self, uid: u32) {
        self.uid = Some(uid);
    }


}

impl Exporter for OptimizerExporter {
    type Export = Export;

    fn read_command(&mut self, command: GameCommand) {
        match command.command_id {
            _ => {
                trace!(
                    command_id = command.command_id,
                    tag = command.get_command_name(),
                    "ignored"
                );
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.uid.is_some()
            && !self.achievements.is_empty()
    }

    #[instrument(skip_all)]
    fn export(mut self) -> Self::Export {
        info!("exporting collected data");

        if self.uid.is_none() {
            warn!("uid was not recorded");
        }

        if self.achievements.is_empty() {
            warn!("relics were not recorded");
        }

        Export {
            source: "artifactarium_archiver",
            build: env!("CARGO_PKG_VERSION"),
            version: 4,
            uid: self.uid,
            achievements: self.achievements,
        }
    }
}

fn format_location(avatar_id: u32) -> String {
    if avatar_id == 0 {
        "".to_owned()
    } else {
        avatar_id.to_string()
    }
}