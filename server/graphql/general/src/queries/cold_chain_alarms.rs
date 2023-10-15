use async_graphql::*;
use graphql_core::ContextExt;
use graphql_types::types::SensorNode;
use repository::{
    EqualFilter, Pagination, SensorFilter, SensorRepository, Sort, TemperatureLogFilter,
    TemperatureLogRepository, TemperatureLogSortField,
};

#[derive(SimpleObject)]
pub struct EmdSettingsNode {
    pub ip: String,
    pub interval_seconds: i32,
}

#[derive(Enum, Copy, Clone, PartialEq, Eq, Debug)]
pub enum AlarmVariant {
    DoorOpen,
    PowerOff,
    TemperatureExcursion,
}

#[derive(SimpleObject)]
pub struct AlarmNode {
    sensor: SensorNode,
    alarm_variant: AlarmVariant,
    temperature: Option<f64>,
}

pub(crate) fn cold_chain_alarms(ctx: &Context<'_>) -> Result<Vec<AlarmNode>> {
    // Auth ?
    // By store id

    let service_provider = ctx.service_provider();
    let service_context = service_provider.basic_context()?;

    let sensors =
        SensorRepository::new(&service_context.connection).query_by_filter(SensorFilter::new())?;
    let mut alarms = Vec::new();

    for sensor in sensors {
        let sensor_filter = SensorFilter::new().id(EqualFilter::equal_to(&sensor.sensor_row.id));

        if let Some(latest_temperature_log) =
            TemperatureLogRepository::new(&service_context.connection)
                .query(
                    Pagination::one(),
                    Some(TemperatureLogFilter::new().sensor(sensor_filter)),
                    Some(Sort {
                        key: TemperatureLogSortField::Datetime,
                        desc: Some(true),
                    }),
                )?
                .pop()
        {
            if latest_temperature_log.temperature_log_row.temperature > 8.0
                || latest_temperature_log.temperature_log_row.temperature < 2.0
            {
                alarms.push(AlarmNode {
                    sensor: SensorNode::from_domain(sensor.clone()),
                    alarm_variant: AlarmVariant::TemperatureExcursion,
                    temperature: Some(latest_temperature_log.temperature_log_row.temperature),
                });
            }
        }

        if sensor.sensor_row.temperature == 10.0 {
            alarms.push(AlarmNode {
                sensor: SensorNode::from_domain(sensor.clone()),
                alarm_variant: AlarmVariant::DoorOpen,
                temperature: None,
            });
            continue;
        }
        if sensor.sensor_row.temperature == 100.0 {
            alarms.push(AlarmNode {
                sensor: SensorNode::from_domain(sensor.clone()),
                alarm_variant: AlarmVariant::PowerOff,
                temperature: None,
            });
            continue;
        }

        if sensor.sensor_row.temperature == 110.0 {
            alarms.push(AlarmNode {
                sensor: SensorNode::from_domain(sensor.clone()),
                alarm_variant: AlarmVariant::PowerOff,
                temperature: None,
            });
            alarms.push(AlarmNode {
                sensor: SensorNode::from_domain(sensor),
                alarm_variant: AlarmVariant::DoorOpen,
                temperature: None,
            });
            continue;
        }
    }
    Ok(alarms)
}
