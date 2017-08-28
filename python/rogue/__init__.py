import sys
from PyQt5.QtWidgets import QApplication, QWidget, QPushButton

class Rogue(QWidget):
  def __init__(self):
    super().__init__()

    self.initUI()

  def initUI(self):
    self.setGeometry(300, 300, 800, 600)
    self.setWindowTitle('Rogue')

    btn = QPushButton('Button', self)
    btn.setToolTip('This is a <b>QPushButton</b> widget')
    btn.resize(btn.sizeHint())
    btn.move(50, 50)

    btn.clicked.connect()


    self.show()


if __name__ == '__main__':
  app = QApplication(sys.argv)

  rogue = Rogue()

  sys.exit(app.exec_())