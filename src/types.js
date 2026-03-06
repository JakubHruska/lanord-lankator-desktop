/**
 * @typedef {'installer' | 'ready_to_play' | 'patch' | 'other'} PackageType
 */

/**
 * @typedef {Object} ManifestData
 * @property {string} slug
 * @property {PackageType} type
 * @property {string} title
 * @property {string} description
 * @property {number} file_size
 * @property {string} archive_file
 * @property {string|null} created_at
 */

/**
 * @typedef {Object} Package
 * @property {number} id
 * @property {string} title
 * @property {PackageType} type
 * @property {string} slug
 * @property {string} description
 * @property {string} archive_file
 * @property {number} file_size
 * @property {string} created_at
 * @property {ManifestData} manifest_data
 */

/**
 * @typedef {'installing' | 'ready'} PackageStatus
 */

/**
 * @typedef {Object} InstalledPackage
 * @property {string} slug
 * @property {PackageType} type
 * @property {string} title
 * @property {string} description
 * @property {number} file_size
 * @property {string} archive_file
 * @property {string|null} created_at
 * @property {PackageStatus} status
 * @property {string} package_path  - absolutní cesta ke složce balíčku
 */

export {};