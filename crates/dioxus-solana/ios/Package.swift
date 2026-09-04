// swift-tools-version:5.9

import PackageDescription

let package = Package(
    name: "PhantomPlugin",
    platforms: [
        .iOS(.v15),
    ],
    products: [
        .library(
            name: "PhantomPlugin",
            type: .static,
            targets: ["PhantomPlugin"]
        )
    ],
    targets: [
        .target(
            name: "PhantomPlugin",
            path: "Sources",
            linkerSettings: [
                .linkedFramework("UIKit"),
                .linkedFramework("Foundation"),
            ]
        )
    ]
)
