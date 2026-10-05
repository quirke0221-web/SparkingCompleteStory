# Product Requirements Document (PRD)

**Project:** Sparking ZERO: Complete Story  
**Target Game:** *Dragon Ball: Sparking! ZERO* (PC / Steam)  
**Status:** Approved Foundation  
**Version:** 1.0  

---

## 1. Problem Statement

In *Dragon Ball: Sparking! ZERO*, the story mode (**Episode Battle**) divides the narrative across separate character-specific campaigns (Goku, Vegeta, Gohan, Piccolo, etc.). Each character only experiences their own battles, resulting in large narrative leaps and disjointed storytelling:
* Key battles fought by supporting characters are skipped entirely when playing as a primary character (e.g., playing as Goku skips the Saiyan invasion battles fought by Piccolo, Gohan, and Krillin).
* Players must exit to the menu and switch between separate character campaigns to piece together the overarching storyline.

---

## 2. Product Vision

**Sparking ZERO: Complete Story** introduces a dedicated **Complete Story** campaign option within the Episode Battle mode. 

The mod stitches together all canonical *Dragon Ball Z* and *Dragon Ball Super* cutscenes, dialogues, and battles into one seamless, unified story experience. As the timeline moves forward, the game naturally shifts the player's perspective to the canonically appropriate character for each encounter.

---

## 3. Core Principles

1. **100% Vanilla Asset Reuse:**  
   The mod relies strictly on the game's existing assets (cutscenes, audio tracks, dialogues, battle setups, and animations). No custom models, voice acting, or animations are required.
2. **Strict Canonical Timeline:**  
   The campaign focuses purely on the canonical timeline. Alternate "What-If" branching paths (the "Sparking!" divergence routes) are excluded to ensure a linear, cohesive, and authentic story flow.
3. **Non-Destructive Coexistence:**  
   All original 12 stock character campaigns must remain untouched and fully accessible at all times.
4. **Save Data Integrity:**  
   The mod must never corrupt, reset, or invalidate the player's vanilla save data, character unlocks, player level, or achievements.
5. **Standard Community Modding Practices:**  
   The mod should follow standard Unreal Engine 5 modding conventions and community tooling so users can install and manage it seamlessly using standard mod managers (e.g., Unverum).

---

## 4. User Experience & What Success Looks Like

* **Discovery:** The player navigates to `Episode Battle` from the main menu and sees a distinct **Complete Story** entry alongside the original character options.
* **Selection:** Selecting Complete Story presents standard campaign options: starting a **New Game** or choosing **Continue** to resume an existing playthrough.
* **Playthrough:** Starting the campaign places the player into the chronological narrative:
  * Begins with Goku during the Raditz encounter.
  * Concluding an encounter smoothly advances the narrative to the next canonical perspective without dropping the player back to the main character selection menu.
  * Covers the canonical events of *Dragon Ball Z* and *Dragon Ball Super*.

---

## 5. Scope Boundaries

### In-Scope
* A dedicated "Complete Story" entry in the Episode Battle menu.
* Seamless perspective-switching between playable characters as the narrative dictates.
* A curated chronological playlist of vanilla battles, cinematics, and cutscenes.
* Campaign progress tracking that allows resuming where the player left off.
* Clean distribution compatible with common mod managers (e.g., Unverum).

### Out-of-Scope (Non-Goals)
* Custom voice acting, custom 3D animations, or fan-made cutscenes.
* Alternate "What-If" routes or branching narrative decisions.
* Modifications to multiplayer, local versus, tournament mode, or custom battles.
* Modifying, replacing, or removing any of the 12 original character campaigns.
