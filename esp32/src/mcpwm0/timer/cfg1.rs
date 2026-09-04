#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum START {
    #[doc = "0: If started then run until timer equal zero"]
    IfRunningStopAtTez = 0,
    #[doc = "1: If started then run until timer equal period"]
    IfRunningStopAtTep = 1,
    #[doc = "2: Start and run"]
    StartAndRun        = 2,
    #[doc = "3: Start and run until timer equal zero"]
    StartThenStopAtTez = 3,
    #[doc = "4: Start and run until timer equal period"]
    StartThenStopAtTep = 4,
}
impl From<START> for u8 {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for START {
    type Ux = u8;
}
impl crate::IsEnum for START {}
#[doc = "Field `START` reader - "]
pub type START_R = crate::FieldReader<START>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<START> {
        match self.bits {
            0 => Some(START::IfRunningStopAtTez),
            1 => Some(START::IfRunningStopAtTep),
            2 => Some(START::StartAndRun),
            3 => Some(START::StartThenStopAtTez),
            4 => Some(START::StartThenStopAtTep),
            _ => None,
        }
    }
    #[doc = "If started then run until timer equal zero"]
    #[inline(always)]
    pub fn is_if_running_stop_at_tez(&self) -> bool {
        *self == START::IfRunningStopAtTez
    }
    #[doc = "If started then run until timer equal period"]
    #[inline(always)]
    pub fn is_if_running_stop_at_tep(&self) -> bool {
        *self == START::IfRunningStopAtTep
    }
    #[doc = "Start and run"]
    #[inline(always)]
    pub fn is_start_and_run(&self) -> bool {
        *self == START::StartAndRun
    }
    #[doc = "Start and run until timer equal zero"]
    #[inline(always)]
    pub fn is_start_then_stop_at_tez(&self) -> bool {
        *self == START::StartThenStopAtTez
    }
    #[doc = "Start and run until timer equal period"]
    #[inline(always)]
    pub fn is_start_then_stop_at_tep(&self) -> bool {
        *self == START::StartThenStopAtTep
    }
}
#[doc = "Field `START` writer - "]
pub type START_W<'a, REG> = crate::FieldWriter<'a, REG, 3, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "If started then run until timer equal zero"]
    #[inline(always)]
    pub fn if_running_stop_at_tez(self) -> &'a mut crate::W<REG> {
        self.variant(START::IfRunningStopAtTez)
    }
    #[doc = "If started then run until timer equal period"]
    #[inline(always)]
    pub fn if_running_stop_at_tep(self) -> &'a mut crate::W<REG> {
        self.variant(START::IfRunningStopAtTep)
    }
    #[doc = "Start and run"]
    #[inline(always)]
    pub fn start_and_run(self) -> &'a mut crate::W<REG> {
        self.variant(START::StartAndRun)
    }
    #[doc = "Start and run until timer equal zero"]
    #[inline(always)]
    pub fn start_then_stop_at_tez(self) -> &'a mut crate::W<REG> {
        self.variant(START::StartThenStopAtTez)
    }
    #[doc = "Start and run until timer equal period"]
    #[inline(always)]
    pub fn start_then_stop_at_tep(self) -> &'a mut crate::W<REG> {
        self.variant(START::StartThenStopAtTep)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MOD {
    #[doc = "0: Freeze"]
    Freeze = 0,
    #[doc = "1: Count up"]
    Up     = 1,
    #[doc = "2: Count down"]
    Down   = 2,
    #[doc = "3: Count up and down"]
    UpDown = 3,
}
impl From<MOD> for u8 {
    #[inline(always)]
    fn from(variant: MOD) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MOD {
    type Ux = u8;
}
impl crate::IsEnum for MOD {}
#[doc = "Field `MOD` reader - "]
pub type MOD_R = crate::FieldReader<MOD>;
impl MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MOD {
        match self.bits {
            0 => MOD::Freeze,
            1 => MOD::Up,
            2 => MOD::Down,
            3 => MOD::UpDown,
            _ => unreachable!(),
        }
    }
    #[doc = "Freeze"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == MOD::Freeze
    }
    #[doc = "Count up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == MOD::Up
    }
    #[doc = "Count down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == MOD::Down
    }
    #[doc = "Count up and down"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == MOD::UpDown
    }
}
#[doc = "Field `MOD` writer - "]
pub type MOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MOD, crate::Safe>;
impl<'a, REG> MOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Freeze"]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut crate::W<REG> {
        self.variant(MOD::Freeze)
    }
    #[doc = "Count up"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(MOD::Up)
    }
    #[doc = "Count down"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(MOD::Down)
    }
    #[doc = "Count up and down"]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut crate::W<REG> {
        self.variant(MOD::UpDown)
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("start", &self.start())
            .field("mod_", &self.mod_())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CFG1_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn mod_(&mut self) -> MOD_W<'_, CFG1_SPEC> {
        MOD_W::new(self, 3)
    }
}
#[doc = "PWM TIMERx working mode and start/stop control configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {}
