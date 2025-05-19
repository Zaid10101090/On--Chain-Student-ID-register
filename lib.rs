
use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("Cd2m4RKmvaKmqFgL5LUvh6EmT6W9Q5eEjgigdFEgoGmc");

#[program]
mod student_id_registry {
    use super::*;

    pub fn register_student(ctx: Context<RegisterStudent>, name: String, student_id: String, enrollment_year: u16) -> Result<()> {
        let student = Student {
            name,
            student_id,
            enrollment_year,
        };
        ctx.accounts.student_registry.registry.insert(ctx.accounts.wallet.key().to_string(), student);
        Ok(())
    }

    pub fn verify_student(ctx: Context<VerifyStudent>) -> Result<Student> {
        let wallet_address = ctx.accounts.wallet.key().to_string();
        let registry = &ctx.accounts.student_registry.registry;
        match registry.get(&wallet_address) {
            Some(student) => Ok(student.clone()),
            None => Err(error!(ErrorCode::StudentNotFound)),
        }
    }
}

#[account]
pub struct StudentRegistry {
    pub registry: HashMap<String, Student>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Student {
    pub name: String,
    pub student_id: String,
    pub enrollment_year: u16,
}

#[derive(Accounts)]
pub struct RegisterStudent<'info> {
    #[account(mut)]
    pub student_registry: Account<'info, StudentRegistry>,
    #[account(signer)]
    pub wallet: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyStudent<'info> {
    pub student_registry: Account<'info, StudentRegistry>,
    pub wallet: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Student not found.")]
    StudentNotFound,
}
