import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'package:rinf/rinf.dart';
import './messages/all.dart';
import 'package:google_fonts/google_fonts.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    final theme = ThemeData(
      brightness: Brightness.dark,
      colorScheme: const ColorScheme.highContrastDark(
        primary: Color.fromRGBO(105, 154, 215, 1),
      ),
      useMaterial3: true,
    );

    return MaterialApp(
      title: 'Xinux Manager',
      themeMode: ThemeMode.dark,
      darkTheme: theme.copyWith(
        textTheme: GoogleFonts.interTextTheme(theme.textTheme),
      ),
      home: const MyHomePage(title: 'Xinux Manager'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Row(
          children: [
            SvgPicture.asset(
              "assets/xinux-logo.svg",
              height: 32,
            ),
            const SizedBox(width: 12),
            Text(widget.title),
          ],
        ),
      ),
      body: SingleChildScrollView(
        child: Column(
          children: [
            Row(
              children: [
                Container(
                  height: 60,
                  width: 320,
                  padding: const EdgeInsets.only(top: 12),
                  child: const TextField(
                    decoration: InputDecoration(
                      border: OutlineInputBorder(),
                      labelText: "Search",
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 12),
            SizedBox(
              width: double.infinity,
              child: DataTable(
                columns: const [
                  DataColumn(
                    label: Expanded(
                      child: Text("Name"),
                    ),
                  ),
                  DataColumn(
                    label: Text("Type"),
                  ),
                  DataColumn(
                    label: Text("Broken"),
                  ),
                  DataColumn(
                    label: Text("Insecure"),
                  ),
                  DataColumn(
                    label: Text("Unfree"),
                  ),
                  DataColumn(
                    label: Text("Unsupported"),
                  ),
                ],
                rows: [
                  DataRow(
                    cells: [
                      const DataCell(
                        Text("firefox"),
                      ),
                      const DataCell(
                        Tooltip(
                          message: "Package",
                          child: Icon(Icons.inventory),
                        ),
                      ),
                      DataCell(
                        Checkbox(
                          value: false,
                          onChanged: (bool? value) {},
                        ),
                      ),
                      DataCell(
                        Checkbox(
                          value: false,
                          onChanged: (bool? value) {},
                        ),
                      ),
                      DataCell(
                        Checkbox(
                          value: false,
                          onChanged: (bool? value) {},
                        ),
                      ),
                      DataCell(
                        Checkbox(
                          value: false,
                          onChanged: (bool? value) {},
                        ),
                      ),
                    ],
                  ),
                  // const DataRow(
                  //   cells: [
                  //     DataCell(
                  //       Text("programs.firefox.enable"),
                  //     ),
                  //     DataCell(
                  //       Tooltip(
                  //         message: "Option",
                  //         child: Icon(Icons.settings),
                  //       ),
                  //     ),
                  //   ],
                  // ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
