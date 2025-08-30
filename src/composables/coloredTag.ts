/**
 * Configuration object for tag colors
 */
interface TagColorConfig {
	color: string; // background color
	textColor: string; // text color (neutral dark gray)
	borderColor: string; // border color (slightly darker than background)
}

/**
 * Generates a consistent, professional-looking color configuration for a tag.
 * Same input → same output. Colors are soft, distinct, and readable.
 *
 * @param {string} tag - The tag text
 * @returns {TagColorConfig} - { color, textColor, borderColor }
 */
function getTagColorConfig(tag: string): TagColorConfig {
	// Initialize cache
	if (!getTagColorConfig.cache) {
		getTagColorConfig.cache = {};
	}
	const cache = getTagColorConfig.cache;

	if (cache[tag]) {
		return cache[tag];
	}

	// Hash the string to a deterministic number
	let hash = 0;
	for (let i = 0; i < tag.length; i++) {
		const char = tag.charCodeAt(i);
		hash = ((hash << 5) - hash + char) | 0; // 32-bit integer
	}
	hash = Math.abs(hash);

	// === Generate refined HSL values ===

	// Hue: full range, but used with low saturation
	const hue = hash % 360;

	// Saturation:
	const saturation = `${21 + (hash % 21)}%`;

	// Lightness for background:
	const lightnessBg = `${46 + (hash % 11)}%`;

	// Border: slightly darker than background for subtle depth
	const lightnessBorder = `${52 + (hash % 8)}%`;

	// Text color: neutral dark gray (not colored!) → better readability and elegance
	const textColor = "rgba(0, 0%, 18%,0.6)"; // near-black gray

	// Generate colors
	const color = `hsla(${hue}, ${saturation}, ${lightnessBg}, 0.55)`;
	const borderColor = `hsla(${hue}, ${saturation}, ${lightnessBorder}, 0.7)`;

	const config: TagColorConfig = {
		color,
		textColor,
		borderColor,
	};

	// Cache and return
	cache[tag] = config;
	return config;
}

// Add cache property
getTagColorConfig.cache = {} as Record<string, TagColorConfig>;

export default getTagColorConfig;
