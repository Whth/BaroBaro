# Duplicate Detection & Ordering Undo Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add duplicate mod detection warnings on the dashboard and undo support for drag-and-drop mod reordering.

**Architecture:** Both features are frontend-only. Duplicate detection uses a computed on `enabled_mods` to group by identity and surface duplicates as a warning alert. Ordering undo uses a local stack of shallow-copied mod arrays, snapshot before each SortableJS drop.

**Tech Stack:** Vue 3 Composition API, Naive UI, SortableJS, vue-i18n

## Global Constraints

- No backend changes — all logic lives in Vue components
- No proto changes — `BarotraumaMod` type is used as-is
- Follow existing Naive UI alert pattern (see `updatesAvailable` and `conflicts` alerts in `DashboardView.vue`)
- All user-facing strings must be localized in both `en.json` and `zh.json`
- Mod identity: `steamWorkshopId` when non-zero, `name` when ID is 0 (local mods)

---

### Task 1: Duplicate Detection

**Files:**
- Modify: `src/views/dashboard/DashboardView.vue`
- Modify: `src/locales/en.json`
- Modify: `src/locales/zh.json`

**Interfaces:**
- Consumes: `enabled_mods` (Ref<BarotraumaMod[]>) from `../../invokes.ts` (already imported)
- Produces: `duplicateMods` computed — array of `{ name: string, count: number }` for UI rendering

- [ ] **Step 1: Add locale keys for duplicate detection**

In `src/locales/en.json`, add inside the `"dashboard"` object (after `"selectProfile"`):

```json
"duplicatesFound": "Duplicate Mods Detected",
"duplicatesDescription": "{count} mod(s) appear more than once in your enabled list:"
```

In `src/locales/zh.json`, add the same keys inside `"dashboard"`:

```json
"duplicatesFound": "检测到重复模组",
"duplicatesDescription": "{count} 个模组在启用列表中出现了多次："
```

- [ ] **Step 2: Add duplicate detection computed**

In `src/views/dashboard/DashboardView.vue`, add after the existing `handleProfileChange` function (around line 112):

```ts
const duplicateMods = computed(() => {
	const seen = new Map<string, { name: string; count: number }>();
	for (const mod of enabled_mods.value) {
		const key = mod.steamWorkshopId > 0
			? `ws:${mod.steamWorkshopId}`
			: `name:${mod.name}`;
		const existing = seen.get(key);
		if (existing) {
			existing.count++;
		} else {
			seen.set(key, { name: mod.name, count: 1 });
		}
	}
	return [...seen.values()].filter((entry) => entry.count > 1);
});
```

- [ ] **Step 3: Add duplicate warning alert to template**

In `src/views/dashboard/DashboardView.vue`, insert the following block after the existing `conflicts` alert (after line 41, before the `<n-grid>`):

```vue
<n-alert
    v-if="duplicateMods.length > 0"
    :title="$t('dashboard.duplicatesFound')"
    closable
    style="margin-bottom: 16px"
    type="warning"
>
  <p>{{ $t('dashboard.duplicatesDescription', { count: duplicateMods.length }) }}</p>
  <ul style="margin: 4px 0; padding-left: 20px">
    <li v-for="dup in duplicateMods" :key="dup.name">
      <strong>{{ dup.name }}</strong> (×{{ dup.count }})
    </li>
  </ul>
</n-alert>
```

- [ ] **Step 4: Verify TypeScript compiles**

Run: `npx vue-tsc --noEmit`
Expected: no errors

- [ ] **Step 5: Commit**

```bash
git add src/views/dashboard/DashboardView.vue src/locales/en.json src/locales/zh.json
git commit -m "feat: add duplicate mod detection warning on dashboard"
```

---

### Task 2: Ordering Undo

**Files:**
- Modify: `src/components/dashboard/ModList.vue`
- Modify: `src/locales/en.json`
- Modify: `src/locales/zh.json`

**Interfaces:**
- Consumes: `enabled_mods` (Ref<BarotraumaMod[]>) from `../../invokes.ts` (already imported)
- Produces: `undoStack` ref (local, not exported) — stack of `BarotraumaMod[][]` snapshots

- [ ] **Step 1: Add locale keys for undo**

In `src/locales/en.json`, add inside the `"modList"` object:

```json
"undo": "Undo",
"orderUndone": "Reorder undone"
```

In `src/locales/zh.json`, add the same keys inside `"modList"`:

```json
"undo": "撤销",
"orderUndone": "已撤销排序"
```

- [ ] **Step 2: Add undoStack ref and import**

In `src/components/dashboard/ModList.vue`, add the `undoStack` ref after the existing `listContainer` ref (line 55):

```ts
const undoStack = ref<BarotraumaMod[][]>([]);
```

- [ ] **Step 3: Snapshot before reorder in onEnd**

In the SortableJS `onEnd` callback (line 84), add the snapshot push **before** the splice operations. Replace the `onEnd` body:

```ts
onEnd(evt) {
	const { oldIndex, newIndex } = evt;
	if (oldIndex == null || newIndex == null || oldIndex === newIndex) return;
	// Snapshot current order before modifying
	undoStack.value.push([...enabled_mods.value]);
	const items = [...enabled_mods.value];
	const [moved] = items.splice(oldIndex, 1);
	items.splice(newIndex, 0, moved);
	enabled_mods.value = items;
	orderChanged.value = true;
},
```

- [ ] **Step 4: Add undo function**

In `src/components/dashboard/ModList.vue`, add after the `saveOrder` function:

```ts
function undoOrder() {
	if (undoStack.value.length === 0) return;
	enabled_mods.value = undoStack.value.pop()!;
	message.info(t("modList.orderUndone"));
}
```

- [ ] **Step 5: Clear undoStack on successful save**

In the `saveOrder` function, add `undoStack.value = [];` after `orderChanged.value = false;`:

```ts
orderChanged.value = false;
undoStack.value = [];
```

- [ ] **Step 6: Clear undoStack on unmount**

In the existing `onUnmounted` callback, add:

```ts
onUnmounted(() => {
	sortable?.destroy();
	undoStack.value = [];
});
```

- [ ] **Step 7: Add undo button to template**

Replace the action bar div (lines 27-31) with:

```vue
<div v-if="orderChanged" style="margin-top: 12px; display: flex; justify-content: flex-end; gap: 8px;">
  <n-button v-if="undoStack.length > 0" @click="undoOrder">
    {{ $t('modList.undo') }}
  </n-button>
  <n-button :loading="saving" type="primary" @click="saveOrder">
    {{ $t('modList.saveOrder') }}
  </n-button>
</div>
```

- [ ] **Step 8: Verify TypeScript compiles**

Run: `npx vue-tsc --noEmit`
Expected: no errors

- [ ] **Step 9: Commit**

```bash
git add src/components/dashboard/ModList.vue src/locales/en.json src/locales/zh.json
git commit -m "feat: add undo support for mod reorder drag-and-drop"
```

---

### Task 3: Manual Verification

- [ ] **Step 1: Run dev server and verify duplicate detection**

Run: `pnpm dev`
- Navigate to Dashboard
- If duplicates exist in enabled mods → warning alert appears with mod names
- If no duplicates → no alert visible
- Alert is closable

- [ ] **Step 2: Verify ordering undo**

- Navigate to Dashboard → Mod List
- Drag a mod to a new position → "Undo" button appears alongside "Save Order"
- Click "Undo" → mod returns to previous position, toast shows "Reorder undone"
- Drag again, then click "Save Order" → both buttons disappear, undo stack cleared

- [ ] **Step 3: Verify TypeScript still compiles**

Run: `npx vue-tsc --noEmit`
Expected: no errors
