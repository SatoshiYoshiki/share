table! {
    use diesel::sql_types::*;
    use crate::model::exports::*;
    twilio_phone_number (phone_number)  {
        phone_number -> Varchar,
        purchased_datetime -> Timestamptz,
        phone_number_status -> NumberStatusEnum,
        assigned_datetime -> Timestamptz,
        released_datetime -> Timestamptz,
        sip_user -> Nullable<Varchar>,
        sip_password -> Nullable<Varchar>,
        credential_list -> Nullable<Varchar>,
        credential_list_sid -> Nullable<Varchar>,
        credential_sid -> Nullable<Varchar>,
        update_datetime -> Timestamptz,
        updater_id -> Varchar,
        revision -> Varchar,
    }
}
