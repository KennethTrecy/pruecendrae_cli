use pruecendrae_core::Request as MaintainerRequest;

#[derive(Debug, PartialEq)]
pub enum Request<'a> {
	Create(Vec<(&'a str, &'a [u8])>),
	Maintainer(MaintainerRequest<'a>),
	ForceKill
}
