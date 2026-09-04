#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SYNC_SPEC>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SYNC_SPEC>;
#[doc = "Field `SYNCI_EN` reader - "]
pub type SYNCI_EN_R = crate::BitReader;
#[doc = "Field `SYNCI_EN` writer - "]
pub type SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` reader - "]
pub type SW_R = crate::BitReader;
#[doc = "Field `SW` writer - "]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCO_SEL {
    #[doc = "0: Sync input signal"]
    SyncIn = 0,
    #[doc = "1: Timer equal zero"]
    Tez    = 1,
    #[doc = "2: Timer equal period"]
    Tep    = 2,
}
impl From<SYNCO_SEL> for u8 {
    #[inline(always)]
    fn from(variant: SYNCO_SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYNCO_SEL {
    type Ux = u8;
}
impl crate::IsEnum for SYNCO_SEL {}
#[doc = "Field `SYNCO_SEL` reader - "]
pub type SYNCO_SEL_R = crate::FieldReader<SYNCO_SEL>;
impl SYNCO_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYNCO_SEL> {
        match self.bits {
            0 => Some(SYNCO_SEL::SyncIn),
            1 => Some(SYNCO_SEL::Tez),
            2 => Some(SYNCO_SEL::Tep),
            _ => None,
        }
    }
    #[doc = "Sync input signal"]
    #[inline(always)]
    pub fn is_sync_in(&self) -> bool {
        *self == SYNCO_SEL::SyncIn
    }
    #[doc = "Timer equal zero"]
    #[inline(always)]
    pub fn is_tez(&self) -> bool {
        *self == SYNCO_SEL::Tez
    }
    #[doc = "Timer equal period"]
    #[inline(always)]
    pub fn is_tep(&self) -> bool {
        *self == SYNCO_SEL::Tep
    }
}
#[doc = "Field `SYNCO_SEL` writer - "]
pub type SYNCO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SYNCO_SEL>;
impl<'a, REG> SYNCO_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sync input signal"]
    #[inline(always)]
    pub fn sync_in(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCO_SEL::SyncIn)
    }
    #[doc = "Timer equal zero"]
    #[inline(always)]
    pub fn tez(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCO_SEL::Tez)
    }
    #[doc = "Timer equal period"]
    #[inline(always)]
    pub fn tep(self) -> &'a mut crate::W<REG> {
        self.variant(SYNCO_SEL::Tep)
    }
}
#[doc = "Field `PHASE` reader - "]
pub type PHASE_R = crate::FieldReader<u16>;
#[doc = "Field `PHASE` writer - "]
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHASE_DIRECTION` reader - "]
pub type PHASE_DIRECTION_R = crate::BitReader;
#[doc = "Field `PHASE_DIRECTION` writer - "]
pub type PHASE_DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn synci_en(&self) -> SYNCI_EN_R {
        SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn synco_sel(&self) -> SYNCO_SEL_R {
        SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn phase_direction(&self) -> PHASE_DIRECTION_R {
        PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC")
            .field("synci_en", &self.synci_en())
            .field("sw", &self.sw())
            .field("synco_sel", &self.synco_sel())
            .field("phase", &self.phase())
            .field("phase_direction", &self.phase_direction())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn synci_en(&mut self) -> SYNCI_EN_W<'_, SYNC_SPEC> {
        SYNCI_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, SYNC_SPEC> {
        SW_W::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn synco_sel(&mut self) -> SYNCO_SEL_W<'_, SYNC_SPEC> {
        SYNCO_SEL_W::new(self, 2)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W<'_, SYNC_SPEC> {
        PHASE_W::new(self, 4)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn phase_direction(&mut self) -> PHASE_DIRECTION_W<'_, SYNC_SPEC> {
        PHASE_DIRECTION_W::new(self, 20)
    }
}
#[doc = "PWM TIMERx sync function configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {}
