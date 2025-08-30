![rustc](https://img.shields.io/badge/rustc-1.88.0-blue.svg)
[![codecov](https://codecov.io/gh/averageeucplayer/lost-metrics-simulator/graph/badge.svg?token=HHRGYYUNM2)](https://codecov.io/gh/averageeucplayer/lost-metrics-simulator)
![CI](https://github.com/averageeucplayer/lost-metrics-simulator/actions/workflows/ci.yml/badge.svg)

# 🎮 Lost Metrics Simulator


## 📦 Installation & Setup

### 1️⃣ **Clone the Repository**

```sh
git clone https://github.com/averageeucplayer/lost-metrics-simulator.git
```

### 2️⃣ Add to Cargo.toml

```toml
[dependencies]
lost-metrics-simulator = { git = "https://github.com/averageeucplayer/lost-metrics-simulator" }
```

```
EncounterSimulator
│
├─ Parties
│   └─ Players
│       ├─ Skills (id, cooldown, effects[Damage/Buff/Debuff/Summon/...])
│       ├─ ActiveBuffs (BuffType, duration, expires_at_tick)
│       ├─ StatusEffects
│
├─ Bosses
│   └─ HP, MaxHP
│   └─ ActiveDebuffs (DebuffType, duration, expires_at_tick)
│
├─ Tick loop
│   ├─ Tick bookkeeping
│   │   └─ Remove expired buffs/debuffs
│   │   └─ Decrement cooldowns
│
│   ├─ Spawn Phase (tick == 0)
│   │   └─ Spawn players
│   │   └─ Spawn bosses
│
│   ├─ Player Phase
│   │   └─ For each player:
│   │       └─ Iterate skills
│   │           └─ If skill.is_ready(tick):
│   │               ├─ For each effect in skill.effects:
│   │               │   ├─ Damage → call calculate_damage (hit_flag, crit, buffs, debuffs, hit_option)
│   │               │   │            apply damage to boss.cur_hp
│   │               │   ├─ Buff → push BuffInstance { buff_type, expires_at_tick }
│   │               │   ├─ Debuff → apply DebuffInstance to boss
│   │               │   ├─ Summon → spawn temporary NPC (deals damage until duration expires)
│   │               │   ├─ Projectile/Trap → queue Action for delayed resolution
│   │               │
│   │               └─ Emit corresponding EncounterEvent packets
│
│   ├─ Special Phase
│   │   └─ Every 180s → leader may cast Sidereal
│   │       └─ Summon powerful NPC (fixed damage skill, duration 30s)
│
│   ├─ Event Emission
│   │   └─ Collect all EncounterEvents from this tick
│
│   └─ Termination Check
│       └─ is_finished() if
│           ├─ tick exceeds cap (e.g. 10k)
│           └─ all bosses.cur_hp <= 0
```