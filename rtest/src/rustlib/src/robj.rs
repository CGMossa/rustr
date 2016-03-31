use super::*;

// #[rustr_export]
pub fn charvec_at(x:CharVec,y:usize)->RResult<String>{
    x.at(y)
}

// #[rustr_export]
pub fn numvec_at(x:NumVec,y:usize)->f64{
    x.at(y).unwrap_or(0.0)
}

// vector ----------------------------

// #[rustr_export]
pub fn clist()-> RResult<RList>{

    Ok(rlist!("sd","Sd"))
}

// #[rustr_export]
pub fn nlist()-> RResult<RList>{

    Ok(rlist!(sd ~ "sd", Sd ~ "Sd"))
}

// #[rustr_export]
pub fn uclist()-> RList{

    unsafe{urlist!("sd","Sd")}
}

// #[rustr_export]
pub fn unlist()-> RList{

    unsafe{urlist!(sd ~ "sd", Sd ~ "Sd")}
}

// charvec

// #[rustr_export]
pub fn charvec()-> RResult<CharVec>{

    Ok(charvec!("sd","Sd"))
}

// #[rustr_export]
pub fn ncharvec()-> RResult<CharVec>{

    Ok(charvec!(sd ~ "sd", Sd ~ "Sd"))
}

// #[rustr_export]
pub fn ucharvec()-> CharVec{

    unsafe{ucharvec!("sd","Sd".into())}
}

// #[rustr_export]
pub fn uncharvec()-> CharVec{

    unsafe{ucharvec!(sd ~ "sd", Sd ~ "Sd")}
}

// boolvec

// #[rustr_export]
pub fn boolvec()-> BoolVec{

    {boolvec!(true,false)}
}

// #[rustr_export]
pub fn nboolvec()-> BoolVec{

    {boolvec!(sd ~ true, Sd ~ false)}
}

// numvec

// #[rustr_export]
pub fn numvec()-> NumVec{

    {numvec!(1.0,2.0)}
}

// #[rustr_export]
pub fn nnumvec()-> NumVec{

    {numvec!(sd ~ 1.0, Sd ~ 3.0)}
}