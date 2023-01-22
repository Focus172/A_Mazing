package Interfaces

import java.awt.image.BufferedImage
import java.io.File
import javax.imageio.ImageIO

interface ImageHelper {
    companion object {
        @kotlin.jvm.JvmStatic
        fun readImage(imagePath: String?, width: Int, height: Int): BufferedImage? {
            try {
                val input_file = File(imagePath)
                var temp: BufferedImage? = BufferedImage(width, height, BufferedImage.TYPE_INT_RGB)
                temp = ImageIO.read(input_file)
                println("Reading complete.")
                return temp
            } catch (e: Exception) {
                println("Error: $e")
            }
            return null
        }

        fun printMaze(mazeWall: Array<BooleanArray>) {
            for (r in mazeWall.indices) {
                for (c in mazeWall[r].indices) {
                    if (mazeWall[r][c]) {
                        print("  ")
                    } else {
                        print("# ")
                    }
                }
                println()
            }
        }

        @kotlin.jvm.JvmStatic
        fun convertToBoolean(image: BufferedImage): Array<BooleanArray>? {
            val width = image.width
            val height = image.height
            val result = Array(height) { BooleanArray(width) }
            for (r in 0 until height) {
                for (c in 0 until width) {
                    if (image.getRGB(c, r) == -1) {
                        result[r][c] = true
                    } else result[r][c] = false
                }
            }
            return result
        }
    }
}