import "dart:convert";

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:json_annotation/json_annotation.dart';

part 'main.g.dart';
//dart run build_runner build
//flutter pub run build_runner build

void main() {
  runApp(const LengWurdsApp());
}

class LengWurdsApp extends StatelessWidget {
  const LengWurdsApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'LengWurds',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Words list'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

@JsonSerializable()
class Lang {
  String lang = "";
  Lang({required this.lang});

  factory Lang.fromJson(Map<String, dynamic> json) => _$LangFromJson(json);

  Map<String, dynamic> toJson() => _$LangToJson(this);
}

@JsonSerializable()
class Word {
  Lang lang = Lang(lang: "");
  String word = "";
  Word({required this.word, required this.lang});

  factory Word.fromJson(Map<String, dynamic> json) => _$WordFromJson(json);

  Map<String, dynamic> toJson() => _$WordToJson(this);
}

@JsonSerializable()
class Translate {
  Word word = Word(word: "", lang: Lang(lang: ""));
  List<Word> translates = <Word>[];
  Translate({required this.word, required this.translates});

  factory Translate.fromJson(Map<String, dynamic> json) =>
      _$TranslateFromJson(json);

  Map<String, dynamic> toJson() => _$TranslateToJson(this);
}

class _MyHomePageState extends State<MyHomePage> {
  List<Translate> _translates = <Translate>[];
  String _current_word = "";
  List<Lang> _langs = <Lang>[];

  void _fetchTranslates() {
    var url = Uri.parse("http://127.0.0.1:6832/api/words?user_id=");
    void process(value) {
      var trs = <Translate>[];
      List<dynamic> trs_ = jsonDecode(value);
      for (var tr in trs_) {
        trs.add(Translate.fromJson(tr));
      }
      setState(() {
        _translates = trs;
      });
    }

    http.read(url).then((value) => process(value)).catchError((e) => print(e));
  }

  void _setCurrentWord(String word) {
    setState(() {
      _current_word = word;
    });
  }

  ListView _buildTranslatesList() {
    if (_translates.length == 0) {
      return ListView(
          shrinkWrap: true,
          padding: const EdgeInsets.all(8),
          children: <Widget>[
            Container(
              height: 50,
              color: Colors.amber[600],
              child: const Center(child: Text('No translates')),
            )
          ]);
    }
    return ListView.builder(
        shrinkWrap: true,
        padding: const EdgeInsets.all(8),
        itemCount: _translates.length,
        itemBuilder: (BuildContext context, int index) {
          var lang = _translates[index].word.lang.lang.toUpperCase();
          var word = _translates[index].word.word;
          return GestureDetector(
            onTap: () => _setCurrentWord(_translates[index].word.word),
            child: Container(
              height: 50,
              child: Center(child: Text('$lang $word')),
            ),
          );
        });
  }

  Widget _currentWord() {
    if (_current_word == "") {
      return Text("Not selected");
    }
    var ws = <Widget>[];
    for (var tr in _translates) {
      if (tr.word.word != _current_word) {
        continue;
      }
      var w = tr.word.word;
      var l = tr.word.lang.lang.toUpperCase();
      ws.add(Text("$l $w"));
      for (var w in tr.translates) {
        var ww = w.word;
        var ll = w.lang.lang.toUpperCase();
        ws.add(Text("$ll $ww"));
      }
      break;
    }
    if (ws.isEmpty) {
      return Text("Not selected");
    }

    return Column(children: ws);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
          child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Container(
                width: 400,
                padding: EdgeInsets.all(15),
                child: Center(child: _buildTranslatesList()),
              ),
            ],
          ),
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Container(
                width: 800,
                child: Center(
                  child: _currentWord(),
                ),
              )
            ],
          ),
        ],
      )),
      floatingActionButton: FloatingActionButton(
          onPressed: _fetchTranslates,
          tooltip: 'Update words list',
          child: const Icon(Icons.update)),
    );
  }
}
