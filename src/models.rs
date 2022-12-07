use diesel::prelude::*;
use crate::schema::*;
use chrono::NaiveDateTime;

///////////////////////////////////////////////////////////////////////////////
//                           USER, ROLES, USERROLES                          //
///////////////////////////////////////////////////////////////////////////////


#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = blueayachanuser)]
pub struct NewBACUser<'a>
{
    pub user_nick: &'a str,
    pub num_commands: &'a i32,
    pub date_added: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = blueayachanuser)]
pub struct BACUser
{
    pub id: i32,
    pub user_nick: String,
    pub num_commands: i32,
    pub date_added: NaiveDateTime,
}
/*
last_pasta TIMESTAMP DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
last_pic TIMESTAMP DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
*/

#[derive(Insertable)]//, Identifiable)]
#[diesel(table_name = roles)]
pub struct NewRole<'a>
{
    pub role_name: &'a str,
    pub date_added: &'a NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = roles)]
pub struct Role
{
    pub id: i32,
    pub role_name: String,
    pub date_added: NaiveDateTime,
}


//#[diesel(belongs_to(i32, foreign_key = user_id))]
//#[diesel(belongs_to(roles, foreign_key = role_id))]
#[diesel(belongs_to(BACUser, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[derive(Insertable, Associations)]
#[diesel(table_name = blueayachanuser_roles)]
pub struct NewBAC_User_Role<'a>
{
    pub user_id: &'a i32,
    pub role_id: &'a i32,
    pub created: &'a NaiveDateTime
}

#[derive(Queryable, Selectable, Associations)]
//#[diesel(belongs_to(blueayachanuser, foreign_key = user_id))]
//#[diesel(belongs_to(roles, foreign_key = role_id))]
#[diesel(belongs_to(BACUser, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(table_name = blueayachanuser_roles)]
pub struct BAC_User_Role
{
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created: NaiveDateTime,
}

///////////////////////////////////////////////////////////////////////////////
//                              DREAMBOUMTWEETS                              //
///////////////////////////////////////////////////////////////////////////////

#[derive(Insertable)]
#[diesel(table_name = dreamboumtweets)]
pub struct New_DBTweet<'a>
{
    pub tweet: &'a str,
    pub tweet_date: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = dreamboumtweets)]
pub struct DBTweet
{
    pub id: i32,
    pub tweet: String,
    pub tweet_date: String,
}

///////////////////////////////////////////////////////////////////////////////
//                           GACHA COMMANDS RELATED                          //
///////////////////////////////////////////////////////////////////////////////

#[derive(Insertable)]
#[diesel(table_name = nocturnedemons)]
pub struct New_NDemon<'a>
{
    pub demon_name: &'a str,
    pub demon_img_link: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = nocturnedemons)]
pub struct NDemon
{
    pub id: i32,
    pub demon_name: String,
    pub demon_img_link: String,
}

macro_rules! generate_simple_db_structs
{
    ($db_name:ident,
    $new_struct_t:ident,
    $struct_t:ident) =>
    {
        #[derive(Insertable)]
        #[diesel(table_name = $db_name)]
        pub struct $new_struct_t<'a>
        {
            pub name: &'a str,
        }
        #[derive(Queryable, Selectable)]
        #[diesel(table_name = $db_name)]
        pub struct $struct_t
        {
            pub id: i32,
            pub name: String,
        }
    };
}
generate_simple_db_structs!(hornedanimes, New_HornedAnime, HornedAnime);
/*generate_simple_db_structs!(meltys, New_Melty, Melty);
generate_simple_db_structs!(luminas, New_Lumina, Lumina);
generate_simple_db_structs!(meltys, New_Melty, Melty);
generate_simple_db_structs!(melees, New_Melee, Melee);
generate_simple_db_structs!(sokus, New_Soku, Soku);
generate_simple_db_structs!(bbcfs, New_BBCF, BBCF);
generate_simple_db_structs!(ggxxacplusrs, New_GGXXACPLUSR, GGXXACPLUSR);
generate_simple_db_structs!(akbs, New_AKB, AKB);
generate_simple_db_structs!(vsavs, New_Vsav, Vsav);*/

/*
#[derive(Insertable)]
#[diesel(table_name = hornedanimes)]
pub struct New_HornedAnime<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = hornedanimes)]
pub struct HornedAnime
{
    pub id: i32,
    pub name: String,
}
*/
#[derive(Insertable)]
#[diesel(table_name = meltys)]
pub struct New_Melty<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = meltys)]
pub struct Melty
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = luminas)]
pub struct New_Lumina<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = luminas)]
pub struct Lumina
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = melees)]
pub struct New_Melee<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = melees)]
pub struct Melee
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = sokus)]
pub struct New_Soku<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = sokus)]
pub struct Soku
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = bbcfs)]
pub struct New_BBCF<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = bbcfs)]
pub struct BBCF
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = ggxxacplusrs)]
pub struct New_GGXXACPLUSR<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = ggxxacplusrs)]
pub struct GGXXACPLUSR
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = akbs)]
pub struct New_AKB<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = akbs)]
pub struct AKB
{
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = vsavs)]
pub struct New_Vsav<'a>
{
    pub name: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = vsavs)]
pub struct Vsav
{
    pub id: i32,
    pub name: String,
}