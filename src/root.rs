use crate::config::Config;
use users::switch::{set_effective_gid, set_effective_uid, switch_user_group};

fn setgroups(groups: &[users::Group]) -> Result<(), std::io::Error> {
    let gids: Vec<libc::gid_t> = groups.iter().map(|group| group.gid()).collect();
    let res = unsafe {
        libc::setgroups(
            gids.len() as libc::size_t,
            gids.as_ptr() as *const libc::gid_t,
        )
    };

    match res {
        0 => Ok(()),
        -1 => Err(std::io::Error::last_os_error()),
        n => unreachable!("setegid returned {}", n),
    }
}

pub fn user_is_root() -> bool {
    users::get_effective_uid() == 0
}

pub fn execute_as_root<F, R>(func: F) -> Result<R, std::io::Error>
where
    F: FnOnce() -> R,
{
    let guard = switch_user_group(0, 0)?;
    let result = func();
    std::mem::drop(guard);
    Ok(result)
}

pub fn drop_root(config: &Config) -> Result<(), String> {
    if !user_is_root() {
        return Ok(());
    }

    let username = config
        .user
        .as_ref()
        .ok_or_else(|| "No user provided".to_string())?;
    let uid = users::get_user_by_name(&username)
        .ok_or_else(|| format!("No user {} found", username))?
        .uid();

    let groupname = config
        .group
        .as_ref()
        .ok_or_else(|| "No group provided".to_string())?;
    let gid = users::get_group_by_name(&groupname)
        .ok_or_else(|| format!("No group {} found", groupname))?
        .gid();

    //Set supplementary groups
    let sup_groups: Vec<users::Group> = users::get_user_groups(&username, gid)
        .unwrap_or_default()
        .into_iter()
        .filter(|group| gid == 0 || group.gid() != 0)
        .collect();
    setgroups(&sup_groups).or_else(|err| Err(err.to_string()))?;

    //Drop group first as it may not possible to do after dropping user
    set_effective_gid(gid).or_else(|err| Err(err.to_string()))?;
    set_effective_uid(uid).or_else(|err| Err(err.to_string()))?;

    Ok(())
}
