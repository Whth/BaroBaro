// Interface defining the structure of a mod item in the queue
import { WorkshopItem } from "../proto/workshop.ts";

export function getStatusType(
	status: ModStatus,
): "default" | "warning" | "success" | "error" {
	switch (status) {
		case ModStatus.Pending:
			return "default";
		case ModStatus.Downloading:
			return "warning";
		case ModStatus.Completed:
			return "success";
		case ModStatus.Error:
			return "error";
		default:
			return "default";
	}
}

export enum ModStatus {
	Pending = "pending",
	Downloading = "downloading",
	Completed = "completed",
	Error = "error",
}

export interface ModItem {
	id?: number; // Steam Workshop ID (if available)
	url?: string; // Original URL input (if provided)
	retrieved?: WorkshopItem | null;
	verified?: boolean; // Whether the mod has passed validation
	status: ModStatus; // Current download status
}
