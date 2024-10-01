use std::time::Duration;

use sea_orm::ActiveValue;

use crate::{
    app::model::managed_serial_port::AppOpenSerialPortOptions,
    core::{
        serial::managed_serial_port::CoreReadState,
        state::open_serial_port::{
            CoreDataBits, CoreFlowControl, CoreIncomingPacket, CoreOpenSerialPortOptions,
            CoreOutgoingPacket, CorePacket, CorePacketDirection, CorePacketOrigin, CoreParity,
            CoreStopBits, CoreSubscriptionPacketOrigin,
        },
    },
};

use super::entity::{
    open_options::{ActiveModel as OpenOptionsActiveModel, Model as OpenOptionsModel},
    packet::{ActiveModel as PacketActiveModel, Model as PacketModel},
};

impl From<OpenOptionsModel> for AppOpenSerialPortOptions {
    fn from(model: OpenOptionsModel) -> Self {
        Self {
            tag: model.tag,
            core_options: CoreOpenSerialPortOptions {
                initial_read_state: match model.init_read_state {
                    0 => CoreReadState::Read,
                    1 => CoreReadState::Stop,
                    _ => {
                        tracing::warn!(
                            init_read_state = model.init_read_state,
                            "Unknown initial read state. Returning default"
                        );

                        Default::default()
                    }
                },
                baud_rate: model.baud_rate as u32,
                data_bits: match model.data_bits {
                    0 => CoreDataBits::Five,
                    1 => CoreDataBits::Six,
                    2 => CoreDataBits::Seven,
                    3 => CoreDataBits::Eight,
                    _ => {
                        tracing::warn!(
                            data_bits = model.data_bits,
                            "Unknown data bits. Returning default"
                        );

                        Default::default()
                    }
                },
                flow_control: match model.flow_control {
                    0 => CoreFlowControl::None,
                    1 => CoreFlowControl::Software,
                    2 => CoreFlowControl::Hardware,
                    _ => {
                        tracing::warn!(
                            flow_control = model.flow_control,
                            "Unknown flow control. Returning default"
                        );

                        Default::default()
                    }
                },
                parity: match model.parity {
                    0 => CoreParity::None,
                    1 => CoreParity::Odd,
                    2 => CoreParity::Even,
                    _ => {
                        tracing::warn!(parity = model.parity, "Unknown parity. Returning default");

                        Default::default()
                    }
                },
                stop_bits: match model.stop_bits {
                    0 => CoreStopBits::One,
                    1 => CoreStopBits::Two,
                    _ => {
                        tracing::warn!(
                            stop_bits = model.stop_bits,
                            "Unknown stop bits. Returning default"
                        );

                        Default::default()
                    }
                },
                timeout: Duration::from_millis(model.timeout_milli_secs as u64),
            },
        }
    }
}

impl From<AppOpenSerialPortOptions> for OpenOptionsActiveModel {
    fn from(options: AppOpenSerialPortOptions) -> Self {
        Self {
            tag: ActiveValue::Set(options.tag),
            init_read_state: ActiveValue::Set(match options.core_options.initial_read_state {
                CoreReadState::Read => 0,
                CoreReadState::Stop => 1,
            }),
            baud_rate: ActiveValue::Set(options.core_options.baud_rate as i32),
            data_bits: ActiveValue::Set(match options.core_options.data_bits {
                CoreDataBits::Five => 0,
                CoreDataBits::Six => 1,
                CoreDataBits::Seven => 2,
                CoreDataBits::Eight => 3,
            }),
            flow_control: ActiveValue::Set(match options.core_options.flow_control {
                CoreFlowControl::None => 0,
                CoreFlowControl::Software => 1,
                CoreFlowControl::Hardware => 2,
            }),
            parity: ActiveValue::Set(match options.core_options.parity {
                CoreParity::None => 0,
                CoreParity::Odd => 1,
                CoreParity::Even => 2,
            }),
            stop_bits: ActiveValue::Set(match options.core_options.stop_bits {
                CoreStopBits::One => 0,
                CoreStopBits::Two => 1,
            }),
            timeout_milli_secs: ActiveValue::Set(options.core_options.timeout.as_millis() as i32),
            ..Default::default()
        }
    }
}

/// Port name is not saved in the model. We have to get it with a join.
impl From<(String, PacketModel)> for CorePacket {
    fn from((port_name, model): (String, PacketModel)) -> Self {
        let packet_direction = match (
            model.incoming,
            model.outgioing,
            model.outgoing_direct,
            model.outgoing_broadcast,
            model.outgoing_subscription,
        ) {
            (true, _, _, _, _) => CorePacketDirection::Incoming(CoreIncomingPacket {
                line: model.data.into(),
            }),
            (_, true, Some(true), _, _) => CorePacketDirection::Outgoing(CoreOutgoingPacket {
                bytes: model.data.into(),
                packet_origin: CorePacketOrigin::Direct,
            }),
            (_, true, _, Some(true), _) => CorePacketDirection::Outgoing(CoreOutgoingPacket {
                bytes: model.data.into(),
                packet_origin: CorePacketOrigin::Broadcast,
            }),
            (_, true, _, _, Some(name_from)) => CorePacketDirection::Outgoing(CoreOutgoingPacket {
                bytes: model.data.into(),
                packet_origin: CorePacketOrigin::Subscription(CoreSubscriptionPacketOrigin {
                    name: name_from,
                }),
            }),
            _ => {
                tracing::warn!("Malformed packet. Defaulting to incoming");

                CorePacketDirection::Incoming(CoreIncomingPacket {
                    line: model.data.into(),
                })
            }
        };

        Self {
            packet_direction,
            port_name,
            timestamp_millis: model.timestamp.timestamp_millis() as u64,
        }
    }
}

impl From<CorePacket> for PacketActiveModel {
    fn from(packet: CorePacket) -> Self {
        let (incoming, outgioing, outgoing_direct, outgoing_broadcast, outgoing_subscription, data) =
            match packet.packet_direction {
                CorePacketDirection::Incoming(incoming_packet) => {
                    (true, false, None, None, None, incoming_packet.line.into())
                }
                CorePacketDirection::Outgoing(outgoing_packet) => {
                    match outgoing_packet.packet_origin {
                        CorePacketOrigin::Direct => (
                            false,
                            true,
                            Some(true),
                            None,
                            None,
                            outgoing_packet.bytes.into(),
                        ),
                        CorePacketOrigin::Broadcast => (
                            false,
                            true,
                            None,
                            Some(true),
                            None,
                            outgoing_packet.bytes.into(),
                        ),
                        CorePacketOrigin::Subscription(subscription) => (
                            false,
                            true,
                            None,
                            None,
                            Some(subscription.name),
                            outgoing_packet.bytes.into(),
                        ),
                    }
                }
            };

        Self {
            tag: todo!("Core Packet Tag"),
            timestamp: todo!("Make timestamp in core DatetimeUtc and convert it in tauri_app"),
            incoming: ActiveValue::Set(incoming),
            outgioing: ActiveValue::Set(outgioing),
            outgoing_direct: ActiveValue::Set(outgoing_direct),
            outgoing_broadcast: ActiveValue::Set(outgoing_broadcast),
            outgoing_subscription: ActiveValue::Set(outgoing_subscription),
            data: ActiveValue::Set(data),
            ..Default::default()
        }
    }
}
