
import code.A_Mazing
import javax.swing.JFrame
import javax.swing.SwingUtilities

object Main {
    //this calls the creation of the window when program is ran
    //fixes some issues with threading by using invokeLater
    @JvmStatic
    fun main(args: Array<String>) {
        SwingUtilities.invokeLater { initWindow() }
    }

    private fun initWindow() {
        val window = JFrame("A_Mazing")
        window.defaultCloseOperation = JFrame.EXIT_ON_CLOSE
        window.isResizable = false
        val sc = A_Mazing()
        window.add(sc)
        window.pack()
        window.setLocationRelativeTo(null) //opens in the center of the screen
        window.isVisible = true //makes visible
    }
}