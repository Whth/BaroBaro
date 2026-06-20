# Duplicate Detection & Ordering Undo

**Date:** 2026-06-20
**Status:** Approved for implementation

## Overview

Two small, high-value UX features for the BaroBaro dashboard:

1. **Duplicate Detection** — Warn when the same mod is enabled twice.
2. **Ordering Undo** — Undo accidental drag-and-drop reordering before saving.

Both features are frontend-only. No backend or proto changes required.

---

## Feature 1: Duplicate Detection

### Problem

A user can end up with the same mod enabled twice — e.g. a local copy alongside a Workshop copy, or two entries with different Workshop IDs pointing to the same content. This causes load order confusion and potential game crashes.

### Design

**Detection logic** — a `computed` in `DashboardView.vue`:

```
duplicateMods = group enabled_mods by:
  - steamWorkshopId (when non-zero)
  - name (for local mods where steamWorkshopId === 0)
Any group with > 1 entry → duplicate set.
```

**UI** — a warning `n-alert` on the Dashboard, rendered above the mod list, conditionally when `duplicateMods.length > 0`:

- Type: `warning`
- Title: localized (e.g. "Duplicate Mods Detected")
- Body: lists the duplicate mod names
- Closable (user can dismiss)
- Same pattern as the existing `updatesAvailable` alert

**No auto-removal.** The user decides which copy to keep.

### Files changed

| File | Change |
|------|--------|
| `src/views/dashboard/DashboardView.vue` | Add `duplicateMods` computed, add `<n-alert>` |
| `src/locales/en.json` | Add `dashboard.duplicatesFound`, `dashboard.duplicatesDescription` |
| `src/locales/zh.json` | Same keys, Chinese translation |

---

## Feature 2: Ordering Undo

### Problem

Drag-and-drop reorder in the mod list is destructive — one accidental drop and the user must manually fix the order. There is no way to revert before saving.

### Design

**History stack** — local ref in `ModList.vue`:

```ts
const undoStack = ref<BarotraumaMod[][]>([]);
```

**Snapshot on drop** — in SortableJS `onEnd`, *before* applying the splice:

```ts
undoStack.value.push([...enabled_mods.value]);
// then apply the splice as before
```

Each stack entry is a shallow copy of the full `enabled_mods` array. Arrays are small (tens of mods), so memory is negligible. Full array avoids the local-mod identity problem (local mods have `steamWorkshopId === 0`, so ID-only snapshots would be ambiguous).

**Undo button** — next to the existing "Save Order" button, visible when `undoStack.length > 0`:

- Icon: undo arrow (e.g. `ArrowUndoOutline` from ionicons5)
- Label: localized (e.g. "Undo")
- On click: `enabled_mods.value = undoStack.value.pop()`
- `orderChanged` stays `true` (still unsaved)

**Clear history:**
- On successful save (`saveOrder` succeeds) → clear stack
- On component unmount → clear stack

**No redo.** YAGNI — undo-only covers the accidental-drop case.

### Files changed

| File | Change |
|------|--------|
| `src/components/dashboard/ModList.vue` | Add `undoStack` ref, snapshot in `onEnd`, add undo button, clear on save/unmount |
| `src/locales/en.json` | Add `modList.undo`, `modList.orderUndone` |
| `src/locales/zh.json` | Same keys, Chinese translation |

---

## Scope

- Frontend only — no Rust backend changes, no proto changes
- Both features touch `DashboardView.vue` and `ModList.vue` respectively, plus locale files
- No new dependencies
