export function getSteamWorkshopId(url: string): number | null {
	try {
		const parsedUrl = new URL(url);
		const idStr = parsedUrl.searchParams.get("id");

		if (idStr === null || idStr.trim() === "") {
			return null;
		}

		const idNum = Number(idStr);

		if (!Number.isFinite(idNum) || !Number.isInteger(idNum) || idNum <= 0) {
			return null;
		}
		return idNum;
	} catch (error) {
		console.error("Invalid URL:", error);
		return null;
	}
}
