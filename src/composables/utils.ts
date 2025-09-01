import { openUrl } from "@tauri-apps/plugin-opener";
import { languageToJSON } from "../proto/config.ts";
import { Language } from "@vicons/ionicons5";
import { config } from "../invokes.ts";

export async function openInWorkshop(item_id: number) {
	await openUrl(
		`https://steamcommunity.com/sharedfiles/filedetails/?id=${item_id}`,
	);
}

/**
 * Convert a timestamp in seconds to a date string in YYYY-MM-DD format
 * @param timestamp - Timestamp in seconds (e.g., 1751876035)
 * @returns Formatted date string, such as "2025-07-06"
 */
export function formatTimestampToDate(timestamp: number): string {
	const date = new Date(timestamp * 1000);

	return date
		.toLocaleDateString(
			languageToJSON(config.value.uiConfig?.language ?? Language.English),
			{
				year: "numeric",
				month: "2-digit",
				day: "2-digit",
			},
		)
		.replace(/\//g, "-");
}
